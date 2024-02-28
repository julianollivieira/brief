#[derive(Debug)]
pub enum InvalidPartError {
    IsEmpty,
    ContainsForbiddenCharacter(char),
    ContainsNonAsciiCharacter(char),
}

const FORBIDDEN_CHARS: [char; 12] = ['<', '>', '(', ')', '[', ']', '\\', ',', ';', ':', '@', '"'];

pub fn validate_part(part: &str) -> Result<(), InvalidPartError> {
    if part.is_empty() {
        return Err(InvalidPartError::IsEmpty);
    }

    let c = part.chars().find(|c| !c.is_ascii());
    if let Some(c) = c {
        return Err(InvalidPartError::ContainsNonAsciiCharacter(c));
    }

    let f = part.chars().find(|c| FORBIDDEN_CHARS.contains(c));
    if let Some(f) = f {
        return Err(InvalidPartError::ContainsForbiddenCharacter(f));
    }

    Ok(())
}
