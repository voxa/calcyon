#[cfg(test)]

mod tests {
    use calcyon;

    #[test]
    fn example() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn example2() {
        assert_eq!(calcyon::module_one::add(1, 2), 3);
    }
}
