use {crossterm::terminal, miette::IntoDiagnostic, std::io::Write};

pub(crate) fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.next() {
                Some('[') => {
                    // CSI sequence: skip until final byte (0x40..=0x7E)
                    for c in chars.by_ref() {
                        if ('@'..='~').contains(&c) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    // OSC sequence: skip until ST (ESC \ or BEL)
                    let mut prev = '\0';
                    for c in chars.by_ref() {
                        if c == '\x07' || (prev == '\x1b' && c == '\\') {
                            break;
                        }
                        prev = c;
                    }
                }
                _ => {}
            }
        } else if !c.is_control() {
            width += 1;
        }
    }
    width
}

pub(crate) fn physical_rows(content_width: usize, terminal_width: u16) -> usize {
    let tw = terminal_width as usize;
    if tw == 0 || content_width == 0 {
        return 1;
    }
    content_width.div_ceil(tw)
}

pub(crate) fn term_width() -> u16 {
    terminal::size().map(|(w, _)| w).unwrap_or(80)
}

pub(crate) fn writeln_physical(
    out: &mut std::io::Stdout,
    line: &str,
    tw: u16,
) -> miette::Result<usize> {
    writeln!(out, "{}", line).into_diagnostic()?;
    Ok(physical_rows(visible_width(line), tw))
}
