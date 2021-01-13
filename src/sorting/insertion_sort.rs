pub fn insertion_sort<T: Ord + PartialOrd>(elems: &mut Vec<T>) -> () {
    if elems.len() == 0 {
        panic!("Can't sort an empty vector, try to push some elements to it")
    }
    for i in 1..elems.len() {
        let mut j = i;

        while (j > 0) && (elems[j - 1] > elems[i]) {
            elems.swap(i, j - 1);
            j -= 1;
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut unsorted = vec!(2, 4, 10, 222, 13, 123, 313);
        let sorted = vec!(2, 4, 10, 13, 123, 222, 313);
        insertion_sort(&mut unsorted);

        assert_eq!(sorted, unsorted);
    }

    #[test]
    #[should_panic(expected="Can't sort an empty vector, try to push some elements to it")]
    fn test_sort_empty() {
        let mut empty_vec: Vec<i32> = vec!();

        insertion_sort(&mut empty_vec);
    }
}