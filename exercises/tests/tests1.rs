// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tea_time {
    #[test]
    fn hello() {
        assert!("hello".to_owned() == String::from("hello"))
    }
}