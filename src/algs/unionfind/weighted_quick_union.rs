use super::UnionFind;

pub struct WeightedQuickUnion {
    ids: Vec<u64>,
    sz: Vec<u64>,
}

impl WeightedQuickUnion {
    pub fn new(n: u64) -> WeightedQuickUnion {
        WeightedQuickUnion {
            ids: (0..n).collect(),
            sz: vec![1; n as usize],
        }
    }

    fn root(&self, i: usize) -> usize {
        let mut i = i;
        while i != self.ids[i] as usize {
            i = self.ids[i] as usize
        }

        i
    }
}

impl UnionFind for WeightedQuickUnion {
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);

        if i == j {
            return;
        }

        if self.sz[i] < self.sz[j] {
            self.ids[i] = j as u64;
            self.sz[j] += self.sz[i]
        } else {
            self.ids[j] = i as u64;
            self.sz[i] += self.sz[j]
        }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn ids(&self) -> Vec<u64> {
        self.ids.clone()
    }
}
