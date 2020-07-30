mod union_find;

use union_find::UnionFindWages;

fn main() {
    let mut data = UnionFindWages::new(10);
    println!("BEFORE {:?}", data);

    data.union(4, 3);
    data.union(4, 5);
    println!("AFTER {:?}", data);
}
