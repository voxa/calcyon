// Example function with its unit tests in the same file
pub fn help_message() -> String {
    "\
welcome to voxa :0
    ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_message_contents() {
        assert_ne!(help_message(), "");
    }

}
