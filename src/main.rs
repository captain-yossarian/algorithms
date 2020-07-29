#[derive(Debug, Clone)]
struct UnionFind {
    pub id: Vec<usize>,
    pub count: usize,
}

impl UnionFind {
    fn new(count: usize) -> Self {
        let mut init = Vec::with_capacity(count);
        for elem in 0..count {
            init.push(elem)
        }

        UnionFind { count, id: init }
    }

    fn count(self) -> usize {
        self.count
    }

    fn find(&mut self, index: usize) -> usize {
        self.id[index]
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_id = self.find(p);
        let q_id = self.find(q);
        if p_id != q_id {
            for index in 0..self.id.len() {
                if self.id[index] == p_id {
                    self.id[index] = q_id
                }
            }
            self.count -= 1;
        }
    }
}
fn main() {
    let mut data = UnionFind::new(10);
    data.union(4, 3);
    data.union(3, 8);
    println!("{:?}", data);
}
