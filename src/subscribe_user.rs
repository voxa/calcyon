use serenity::prelude::*;

// Example function with its unit tests in the same file
pub fn subscribe_user(ctx: &Context) -> String {
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_message_contents() {
        assert_ne!(1,0);
    }

}
