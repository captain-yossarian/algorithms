type Value = i32;

#[derive(Debug, Clone)]
pub struct MinHeap {
    items: Vec<Value>,
    size: usize,
}

impl MinHeap {
    pub fn new(capacity: usize) -> Self {
        MinHeap {
            items: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    pub fn left_child_index(parent_index: usize) -> usize {
        2 * parent_index + 1
    }

    pub fn right_child_index(parent_index: usize) -> usize {
        2 * parent_index + 2
    }
    pub fn parent_index(child_index: usize) -> usize {
        (child_index - 1) / 2
    }

    pub fn has_left_child(&self, index: usize) -> bool {
        MinHeap::left_child_index(index) < self.size
    }

    pub fn has_right_child(&self, index: usize) -> bool {
        MinHeap::right_child_index(index) < self.size
    }

    pub fn has_parent(&self, index: usize) -> bool {
        MinHeap::parent_index(index) >= u8::MIN.into()
    }

    pub fn left_child(&self, index: usize) -> Option<&Value> {
        let left_child_index = MinHeap::left_child_index(index);
        self.items.get(left_child_index)
    }

    pub fn right_child(&self, index: usize) -> Option<&Value> {
        let right_child_index = MinHeap::right_child_index(index);
        self.items.get(right_child_index)
    }

    pub fn parent(&self, index: usize) -> Option<&Value> {
        let parent_index = MinHeap::parent_index(index);
        self.items.get(parent_index)
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.items.swap(first, second);
    }

    pub fn peek(&self) -> Option<&Value> {
        self.items.first()
    }

    pub fn poll(&mut self) -> Option<&Value> {
        if let Some(first_item) = self.items.get_mut(0) {

            if let Some(last_item) = self.items.pop() {
                *first_item = last_item;
            }

            self.heapifyDown();
            self.items.first()
        } else {
            None
        }
    }

    pub fn add(&mut self, item: Value) {
        self.items.push(item);
        self.heapifyUp();
    }

    pub fn heapifyDown(&self) {}

    pub fn heapifyUp(&mut self) {
        let mut index = self.items.len() - 1;
        loop {
            if let Some(value) = self.items.get(index) {
                if let Some(parent) = self.parent(index) {
                    let condition = self.has_parent(index) && (parent > value);

                    if !condition {
                        break;
                    }
                    self.swap(MinHeap::parent_index(index), index);
                    index = MinHeap::parent_index(index);
                }
            }
        }
    }
}
///https://www.youtube.com/watch?v=t0Cq6tVNRBA