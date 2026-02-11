use {dyn_clone::DynClone, std::fmt};

pub type CustomUserError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug, PartialEq)]
pub enum Validation {
    Valid,
    Invalid(ErrorMessage),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorMessage {
    Default,
    Custom(String),
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorMessage::Default => write!(f, "Invalid input"),
            ErrorMessage::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        ErrorMessage::Custom(s.to_string())
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        ErrorMessage::Custom(s)
    }
}

pub trait Validate<T: ?Sized>: DynClone {
    fn validate(&self, input: &T) -> Result<Validation, CustomUserError>;
}

impl<T: ?Sized> Clone for Box<dyn Validate<T>> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

impl<T, F> Validate<T> for F
where
    T: ?Sized,
    F: Fn(&T) -> Result<Validation, CustomUserError> + Clone,
{
    fn validate(&self, input: &T) -> Result<Validation, CustomUserError> {
        (self)(input)
    }
}

pub(crate) fn run_validator<T: ?Sized>(
    validator: &dyn Validate<T>,
    input: &T,
) -> Result<(), String> {
    match validator.validate(input) {
        Ok(Validation::Valid) => Ok(()),
        Ok(Validation::Invalid(msg)) => Err(msg.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
