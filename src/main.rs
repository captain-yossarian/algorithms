mod binary_tree;
mod union_find;

use binary_tree::Node;
use union_find::UnionFindWages;

fn main() {
    let mut data = UnionFindWages::new(10);
    let mut root: Node<u8> = Node::new(5);

    root.add(10);
    root.add(2);
    root.add(1);
    root.add(7);
    root.add(6);
    root.add(11);
    println!("Tree {:?}", root.find(8));
}
