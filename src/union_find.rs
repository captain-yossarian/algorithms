#[derive(Debug, Clone)]
pub struct UnionFind {
    pub id: Vec<usize>,
    pub count: usize,
}

impl UnionFind {
    pub fn new(count: usize) -> Self {
        let mut init = Vec::with_capacity(count);
        for elem in 0..count {
            init.push(elem)
        }

        UnionFind { count, id: init }
    }

    pub fn count(self) -> usize {
        self.count
    }

    pub fn find(&mut self, index: usize) -> usize {
        let mut mut_index = index;
        loop {
            if mut_index != self.id[index] {
                mut_index = self.id[index];
            } else {
                return mut_index;
            }
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if p_root != q_root {
            self.id[p_root] = q_root;
        }
        self.count -= 1;
    }
}
