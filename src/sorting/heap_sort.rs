use crate::data_structures::{BinHeap, MaxHeap, MinHeap};

fn return_sorted<R: Ord + PartialOrd, T: BinHeap<R>>(heap: &mut T) -> Vec<R> {
    let mut sorted_vec: Vec<R> = vec!();

    for _ in 0..heap.len() {
        sorted_vec.push(heap.extract().unwrap())
    }

    sorted_vec
}

pub fn heap_sort<T: Ord + PartialOrd>(elems: Vec<T>, desc: bool) -> Vec<T> {
    if elems.len() == 0 {
        panic!("Can't sort an empty vector, try to push some elements to it")
    }
    match desc {
        true => {
            let mut heap = MaxHeap::new(elems);
            heap.build_heap();
            return_sorted(&mut heap)
        } 
        false => {
            let mut heap = MinHeap::new(elems);
            heap.build_heap();

            return_sorted(&mut heap)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_asc() {
        let unsorted = vec!(2, 4, 10, 222, 13, 123, 313);
        let sorted = vec!(2, 4, 10, 13, 123, 222, 313);

        assert_eq!(sorted, heap_sort(unsorted, false))
    }

    #[test]
    fn test_sort_desc() {
        let unsorted = vec!(2, 4, 10, 222, 13, 123, 313);
        let sorted = vec!(313, 222, 123, 13, 10, 4, 2);

        assert_eq!(sorted, heap_sort(unsorted, true))
    }

    #[test]
    #[should_panic(expected="Can't sort an empty vector, try to push some elements to it")]
    fn test_sort_empty() {
        let empty_vec: Vec<i32> = vec!();

        heap_sort(empty_vec, false);
    }
}
