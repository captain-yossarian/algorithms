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
            let value = self.id[index];
            if mut_index != value {
                mut_index = value;
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

#[derive(Debug, Clone)]
pub struct UnionFindWages {
    pub id: Vec<usize>,
    pub sz: Vec<usize>,
    pub count: usize,
}

impl UnionFindWages {
    pub fn new(count: usize) -> Self {
        let mut id = Vec::with_capacity(count);
        let mut sz = Vec::with_capacity(count);

        for elem in 0..count {
            id.push(elem);
            sz.push(elem)
        }

        UnionFindWages { count, id, sz }
    }

    pub fn count(self) -> usize {
        self.count
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn find(&mut self, index: usize) -> usize {
        let mut mut_index = index;
        loop {
            let value = self.id[index];
            if mut_index != value {
                mut_index = value;
            } else {
                return mut_index;
            }
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        if p_root != q_root {
            if self.sz[p_root] < self.sz[q_root] {
                self.id[p_root] = q_root;
                println!("!");
                self.sz[q_root] += self.sz[p_root]
            } else {
                self.id[q_root] = p_root;
                self.sz[p_root] += self.sz[q_root]
            }
        }
        self.count -= 1;
    }
}
