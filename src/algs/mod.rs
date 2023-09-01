use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub enum UnionFindAlgs {
    QuickFind,
}

#[enum_dispatch(UnionFindAlgs)]
pub trait UnionFind {
    fn union(&mut self, p: usize, q: usize);
    fn connected(&self, p: usize, q: usize) -> bool;
    fn ids(&self) -> Vec<u64>;
}

pub struct QuickFind {
    pub ids: Vec<u64>,
}

impl QuickFind {
    pub fn new(n: u64) -> QuickFind {
        QuickFind {
            ids: (0..n).collect(),
        }
    }
}

impl UnionFind for QuickFind {
    fn union(&mut self, p: usize, q: usize) {
        let pid = self.ids[p];
        let qid = self.ids[q];

        // TODO: can this clone be removed?
        for (i, id) in self.ids.clone().iter().enumerate() {
            if *id == pid {
                self.ids[i] = qid;
            }
        }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.ids[p] == self.ids[q]
    }
    fn ids(&self) -> Vec<u64> {
        self.ids.clone()
    }
}
