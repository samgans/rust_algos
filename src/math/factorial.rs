fn _fact_rec(num: u32, acc: u128) -> u128 {
    if num == 1 {
        acc
    } else {
        _fact_rec(num - 1, num as u128 * acc)
    }
}

fn _fact_iter(num: u32) -> u128 {
    let mut product: u128 = 1;
    for counter in 1..num as u128 {
        product = product * (counter + 1);
    }
    product
}

pub fn fact_rec(num: u32) -> u128 {
    if num == 0 {
        0
    } else {
        _fact_rec(num, 1)
    }
}

pub fn fact_iter(num: u32) -> u128 {
    if num == 0 {
        0
    } else {
        _fact_iter(num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fact_rec() {
        assert_eq!(0, fact_rec(0));
        assert_eq!(1, fact_rec(1));
        assert_eq!(2, fact_rec(2));
        assert_eq!(6, fact_rec(3));
        assert_eq!(24, fact_rec(4));
        assert_eq!(120, fact_rec(5));
    }

    #[test]
    fn test_fact_iter() {
        assert_eq!(0, fact_iter(0));
        assert_eq!(1, fact_iter(1));
        assert_eq!(2, fact_iter(2));
        assert_eq!(6, fact_iter(3));
        assert_eq!(24, fact_iter(4));
        assert_eq!(120, fact_iter(5));
    }

    #[test]
    fn test_fact_match() {
        for m in 0..=5 {
            assert_eq!(fact_rec(m), fact_iter(m))
        }
    }
}
