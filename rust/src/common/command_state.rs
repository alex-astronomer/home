use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum CommandState {
    On,
    Off
}

#[derive(Error, Debug)]
pub enum CommandStateError {
    #[error("input bytes must be ON/OFF")]
    InvalidCommandState,
    #[error("error while parsing the state to a string from bytes")]
    ParseIntError(#[from] std::string::FromUtf8Error),
}

impl TryFrom<&[u8]> for CommandState {
    type Error = CommandStateError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let as_string = &String::from_utf8(bytes.to_vec())?[..];
        match as_string {
            "ON" => Ok(CommandState::On),
            "OFF" => Ok(CommandState::Off),
            _ => Err(CommandStateError::InvalidCommandState),
        }
    }
}

impl From<&CommandState> for bool {
    fn from(command_state: &CommandState) -> Self {
        match command_state {
            CommandState::On => true,
            CommandState::Off => false,
        }
    }
}

impl From<bool> for CommandState {
    fn from(bool_: bool) -> Self {
        match bool_ {
            true => CommandState::On,
            false => CommandState::Off,
        }
    }
}

impl std::fmt::Display for CommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandState::On => write!(f, "ON"),
            CommandState::Off => write!(f, "OFF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(input, expected,
        case(true, CommandState::On),
        case(false, CommandState::Off),
    )]
    fn test_from_bool(input: bool, expected: CommandState) {
        assert_eq!(CommandState::from(input), expected);
    }

    #[rstest(input, expected,
        case(&CommandState::On, true),
        case(&CommandState::Off, false)
    )]
    fn test_from_command_state(input: &CommandState, expected: bool) {
        let input_into: bool = input.into();
        assert_eq!(input_into, expected);
    }

    #[rstest(input, expected,
        case("ON".as_bytes(), CommandState::On),
        case("OFF".as_bytes(), CommandState::Off)
    )]
    fn test_try_from_u8(input: &[u8], expected: CommandState) {
        assert_eq!(CommandState::try_from(input).unwrap(), expected);
    }

    #[rstest(input,
        case("ASDF".as_bytes()),
        case(&[0xc3_u8, 0x28_u8]),
    )]
    fn test_try_from_u8_error(input: &[u8]) {
        assert!(CommandState::try_from(input).is_err())
    }

    #[rstest(input, expected,
        case(CommandState::On, "ON"),
        case(CommandState::Off, "OFF"),
    )]
    fn test_to_string(input: CommandState, expected: &str) {
        assert_eq!(input.to_string(), expected);
    }
}
