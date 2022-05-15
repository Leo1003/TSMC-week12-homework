const SET_5_PRICE: f64 = 8.0 * 5.0 * 0.75;
const SET_4_PRICE: f64 = 8.0 * 4.0 * 0.80;
const SET_3_PRICE: f64 = 8.0 * 3.0 * 0.90;
const SET_2_PRICE: f64 = 8.0 * 2.0 * 0.95;
const SET_1_PRICE: f64 = 8.0;

pub fn price(books: &[u8]) -> f64 {
    let mut total = 0.0;
    let mut book_type_count = [0usize; 5];

    for &id in books {
        if id < 5 {
            book_type_count[id as usize] += 1;
        }
    }

    let s5 = get_set(&mut book_type_count, 5);
    let s4 = get_set(&mut book_type_count, 4);
    let s3 = get_set(&mut book_type_count, 3);
    let s2 = get_set(&mut book_type_count, 2);
    let s1 = get_set(&mut book_type_count, 1);

    total += SET_5_PRICE * s5 as f64;
    total += SET_4_PRICE * s4 as f64;
    total += SET_3_PRICE * s3 as f64;
    total += SET_2_PRICE * s2 as f64;
    total += SET_1_PRICE * s1 as f64;

    total
}

fn get_set(book_type_count: &mut [usize], set_type: usize) -> usize {
    if book_type_count.len() != 5 || set_type > 5 {
        panic!();
    }

    let mut set_cnt = 0;

    while book_type_count.iter().copied().filter(|c| *c > 0).count() >= set_type {
        let mut d: Vec<&mut usize> = book_type_count
            .iter_mut()
            .filter(|c| **c > 0)
            .collect();

        d.sort();
        d.truncate(set_type);

        set_cnt += d.iter().map(|r| **r).min().unwrap_or_default();

        for c in d {
            *c = c.saturating_sub(set_cnt);
        }
    }

    set_cnt
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

    #[test]
    fn several_discounts() {
        assert_eq!(23.2, price(&[0, 0, 1]));
        assert_eq!(30.4, price(&[0, 0, 1, 1]));
        assert_eq!(40.8, price(&[0, 0, 1, 2, 2, 3]));
        assert_eq!(38.0, price(&[0, 1, 1, 2, 3, 4]));
    }
}
