pub mod min_heap
{
    pub struct MinHeap<T>
    {
        heap: std::collections::BinaryHeap<std::cmp::Reverse::<T>>
    }

    impl<T> MinHeap<T> where T: Ord
    {
        pub fn new() -> Self
        {
            Self
            {
                heap: std::collections::BinaryHeap::<std::cmp::Reverse::<T>>::new()
            }
        }

        pub fn pop(&mut self) -> Option<T>
        {
            let ret = if let Some(std::cmp::Reverse::<T>(ret)) = self.heap.pop() { Some(ret) } else { None };
            ret
        }

        pub fn push(&mut self, item: T)
        {
            self.heap.push(std::cmp::Reverse::<T>(item));
        }

        pub fn capacity(&self) -> usize
        {
            self.heap.capacity()
        }

        pub fn clear(&mut self)
        {
            self.heap.clear();
        }

        pub fn append(&mut self, other: &mut MinHeap<T>)
        {
            self.heap.append(&mut other.heap);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let arr = vec![3, 2, 4, 1, 5];
        let ans = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let mut q = min_heap::MinHeap::new();
        let mut q2 = min_heap::MinHeap::new();
        for e in arr
        {
            q.push(e);
            q2.push(e);
        }
        q.append(&mut q2);
        let mut it = 0;
        while let Some(x) = q.pop()
        {
            assert_eq!(ans[it], x);
            it += 1;
        }
    }
}