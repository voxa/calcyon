use serenity::prelude::*;

// Example function with its unit tests in the same file
pub fn show_profile(ctx: &Context) -> String {
    println!("eee");
    "\
There is currently no system to store profile sorry lmao
    ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_message_contents() {
        assert_eq!(1,1);
    }

}
