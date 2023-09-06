use super::UnionFind;

pub struct QuickUnion {
    pub ids: Vec<u64>,
}

impl QuickUnion {
    pub fn new(n: u64) -> QuickUnion {
        QuickUnion {
            ids: (0..n).collect(),
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

impl UnionFind for QuickUnion {
    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.root(p);
        self.ids[p_root] = self.root(q) as u64;
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }
    fn ids(&self) -> Vec<u64> {
        self.ids.clone()
    }
}
