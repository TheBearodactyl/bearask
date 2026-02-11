use crate::form::value::FormValue;

pub enum FieldAction {
    Continue,
    Confirm(FormValue),
    Next,
    Prev,
    Cancel,
}
