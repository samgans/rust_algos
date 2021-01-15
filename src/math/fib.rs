fn _fib_rec(n_iter: u32, prev: u128, next: u128) -> u128 {
    if n_iter == 0 {
        prev
    } else {
        _fib_rec(n_iter - 1, next, prev + next)
    }
}

fn _fib_iter(num: u32) -> u128 {
    let mut num_prev: u128 = 0;
    let mut num_next:u128 = 1;
    for i in 0..num - 1 {
        println!("{}", i);
        let sum = num_prev + num_next;
        num_prev = num_next;
        num_next = sum;
    }
    num_next
}

pub fn fib_rec(num: u32) -> u128 {
    _fib_rec(num, 0, 1)
}

pub fn fib_iter(num: u32) -> u128 {
    if num == 0 {
        0
    } else {
        _fib_iter(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_rec() {
        assert_eq!(0, fib_rec(0));
        assert_eq!(1, fib_rec(1));
        assert_eq!(1, fib_rec(2));
        assert_eq!(2, fib_rec(3));
        assert_eq!(5, fib_rec(5));
        assert_eq!(8, fib_rec(6));
    }

    #[test]
    fn test_fib_iter() {
        assert_eq!(0, fib_iter(0));
        assert_eq!(1, fib_iter(1));
        assert_eq!(1, fib_iter(2));
        assert_eq!(2, fib_iter(3));
        assert_eq!(5, fib_iter(5));
        assert_eq!(8, fib_iter(6));
    }

    #[test]
    fn test_fib_match() {
        for m in 0..10 {
            assert_eq!(fib_iter(m), fib_rec(m));
        }
    }
}