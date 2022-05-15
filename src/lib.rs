pub fn price(books: &[u8]) -> f64 {
    let mut total = 0.0;

    for id in books {
        total += 8.0;
    }

    total
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

    #[test]
    fn simple_discounts() {
        assert_eq!(15.2, price(&[0, 1]));
        assert_eq!(21.6, price(&[0, 2, 4]));
        assert_eq!(25.6, price(&[0, 1, 2, 4]));
        assert_eq!(30.0, price(&[0, 1, 2, 3, 4]));
    }
}
