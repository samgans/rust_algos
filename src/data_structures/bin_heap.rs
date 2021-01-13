pub struct MinHeap<R: Ord + PartialOrd>{
    elems: Vec<R>,
}

pub struct MaxHeap<R: Ord + PartialOrd>{
    elems: Vec<R>,
}

pub trait BinHeap<R: Ord + PartialOrd,>{
    fn elems(&mut self) -> &mut Vec<R>;
    fn comparator(left: &R, right: &R) -> bool;

    fn len(&mut self) -> usize {
        self.elems().len()
    }

    fn is_empty(&mut self) -> bool {
        self.elems().is_empty()
    }

    fn sift_down(&mut self, ind: usize) -> () {
        let elems = self.elems();

        let ind_left = (ind * 2) + 1;
        let ind_right = (ind * 2) + 2;

        let num_right = elems.get(ind_right);
        let num_left = elems.get(ind_left);

        let ind_compare = {
            match (num_left, num_right) {
                (Some(one), Some(two)) => match Self::comparator(one, two) {
                    true => ind_left,
                    false => ind_right
                },
                (Some(_), None) => ind_left,
                _ => return ()
            }
        };
    
        if Self::comparator(
            &elems[ind_compare], &elems[ind]
        ) {
            elems.swap(ind, ind_compare);
            self.sift_down(ind_compare);
            self.sift_up(ind);
        }
    }

    fn sift_up(&mut self, ind: usize) -> () {
        let elems = self.elems();

        if ind == 0 as usize {
            return ()
        }
    
        let ind_upper = if let 0 = ind % 2 {
                (ind - 2) / 2
            } else {
                (ind - 1) / 2
            };

        if Self::comparator(&elems[ind], &elems[ind_upper]) {
            elems.swap(ind_upper, ind);
        }
    
        self.sift_up(ind_upper);
        self.sift_down(ind);
    }
    
    fn extract(&mut self) -> Option<R> {
        let elems = self.elems();
        let len = elems.len();

        if len > 0 {
            elems.swap(0, len - 1);
            let extr = elems.pop();
            self.sift_down(0);
            
            extr
        } else {
            None
        }
    }


    fn build_heap(&mut self) -> () {
        let elems = self.elems();
        let len = elems.len();

        for ind in (0..len).rev() {
            self.sift_down(ind)
        }
    }

    fn new(elems: Vec<R>) -> Self;
}

impl<R: Ord + PartialOrd> BinHeap<R> for MaxHeap<R> {
    fn elems(&mut self) -> &mut Vec<R> {
        &mut self.elems
    }

    fn comparator(left: &R, right: &R) ->  bool{
        left > right
    }

    fn new(elems: Vec<R>) -> Self {
        Self {
            elems
        }
    }
}

impl<R: Ord + PartialOrd> BinHeap<R> for MinHeap<R> {
    fn elems(&mut self) -> &mut Vec<R> {
        &mut self.elems
    }

    fn comparator(left: &R, right: &R) ->  bool{
        left < right
    }

    fn new(elems: Vec<R>) -> Self {
        Self {
            elems
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_max() {
        let mut heap = MaxHeap::new(vec!(1, 2, 3));
        heap.build_heap();

        assert_eq!(3, heap.extract().unwrap())

    }

    #[test]
    fn test_build_min() {
        let mut heap = MinHeap::new(vec!(3, 2, 1));
        heap.build_heap();

        assert_eq!(1, heap.extract().unwrap())
    }
}
