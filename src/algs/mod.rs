use enum_dispatch::enum_dispatch;

#[derive(Clone, Default, PartialEq)]
pub enum UnionFindType {
    #[default]
    QuickFind,
    QuickUnion,
}

impl UnionFindType {
    pub fn name(&self) -> String {
        match self {
            UnionFindType::QuickFind => "Quick Find".to_owned(),
            UnionFindType::QuickUnion => "Quick Union".to_owned(),
        }
    }

    pub fn get_alg(&self, n: u64) -> UnionFindAlgs {
        match self {
            UnionFindType::QuickFind => UnionFindAlgs::QuickFind(QuickFind::new(n)),
            UnionFindType::QuickUnion => UnionFindAlgs::QuickUnion(QuickUnion::new(n)),
        }
    }
}

#[enum_dispatch]
pub enum UnionFindAlgs {
    QuickFind,
    QuickUnion,
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
