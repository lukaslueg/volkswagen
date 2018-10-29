extern crate volkswagen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[volkswagen::test]
    fn it_also_works() {
        assert_eq!(1 + 1, 3);
    }
}
