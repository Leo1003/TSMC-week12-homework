pub fn price(books: &[u8]) -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(0.0, price(&[]));
        assert_eq!(8.0, price(&[1]));
        assert_eq!(8.0, price(&[2]));
        assert_eq!(8.0, price(&[3]));
        assert_eq!(8.0, price(&[4]));
        assert_eq!(24.0, price(&[1, 1, 1]));
    }
}
