/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::cmp::PartialOrd + std::cmp::PartialEq + std::clone::Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {    // 先在堆底部添加元素，然后将元素上升 
        //TODO
        self.items.push(value);
        self.count += 1;
        if self.count == 1 {     // 第一个元素不再进行比较 
            return;
        }
        let mut idx = self.count;
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]){                  // 循环向上比较，如果当前元素小于父元素，则交换位置，最多到2
            // std::mem::swap(&mut self.items[idx], &mut self.items[self.parent_idx(idx)]);
            let par = self.parent_idx(idx);
            let tmp = self.items[idx].clone();
            self.items[idx] = self.items[par].clone();
            self.items[par] = tmp;
            idx = self.parent_idx(idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.items.get(self.left_child_idx(idx)) != None && self.items.get(self.right_child_idx(idx)) != None {
            if self.items[self.left_child_idx(idx)] < self.items[self.right_child_idx(idx)] {
                return self.left_child_idx(idx);
            } else {
                return self.right_child_idx(idx);
            }
        } else if self.items.get(self.left_child_idx(idx)) != None && self.items.get(self.right_child_idx(idx)) == None{
            return self.left_child_idx(idx);
        } else {
            return idx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::clone::Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        }
        let anser = self.items[1].clone();                    // 先把第一个元素拿出来
        self.items[1] = self.items.pop().unwrap();               // 把最后一个元素放到第一个位置上 
        self.count -= 1;

        let mut swap_pos: usize = 1;
        while swap_pos*2 <= self.count {                          // 把第一个元素下沉
            let left_child = swap_pos*2;
            let mut right_child = swap_pos*2;
            if swap_pos*2+1 < self.count{
                right_child += 1;
            }
            if (self.comparator)(&self.items[left_child], &self.items[right_child]) {
                right_child = left_child;
            }
            if (self.comparator)(&self.items[right_child], &self.items[swap_pos]){
                let tmp = self.items[swap_pos].clone();         // 交换
                self.items[swap_pos] = self.items[right_child].clone();
                self.items[right_child] = tmp;
                swap_pos = right_child;
            } else {
                break;
            }
        }
        Some(anser)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}