mod binary_search_tree;
mod min_heap;
mod union_find;

use binary_search_tree::Node;
use min_heap::MinHeap;
use union_find::UnionFindWages;

fn main() {
    let mut data = UnionFindWages::new(10);
    let mut root: Node<u8> = Node::new(26);
    let min_heap = MinHeap::new(10);
    println!("Tree {:?}", min_heap.has_parent(1));
}
