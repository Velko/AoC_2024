use crate::InvalidInput;

pub trait ResultExt<T> {
    fn map_err_to_invalid_input(self, input: &str) -> Result<T, InvalidInput>;
}

impl<T, E> ResultExt<T> for Result<T, E> {
    fn map_err_to_invalid_input(self, input: &str) -> Result<T, InvalidInput> {
        self.map_err(|_| InvalidInput(input.to_owned()))
    }
}

impl<T> ResultExt<T> for Option<T> {
    fn map_err_to_invalid_input(self, input: &str) -> Result<T, InvalidInput> {
        self.ok_or(InvalidInput(input.to_owned()))
    }
}
