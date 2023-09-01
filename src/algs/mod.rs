use enum_dispatch::enum_dispatch;

#[derive(Clone, Default, PartialEq)]
pub enum UnionFindType {
    #[default]
    QuickFind,
    QuickUnion,
    WeightedQuickUnion,
}

impl UnionFindType {
    pub fn name(&self) -> String {
        match self {
            UnionFindType::QuickFind => "Quick Find".to_owned(),
            UnionFindType::QuickUnion => "Quick Union".to_owned(),
            UnionFindType::WeightedQuickUnion => "Weighted Quick Union".to_owned(),
        }
    }

    pub fn get_alg(&self, n: u64) -> UnionFindAlgs {
        match self {
            UnionFindType::QuickFind => UnionFindAlgs::QuickFind(QuickFind::new(n)),
            UnionFindType::QuickUnion => UnionFindAlgs::QuickUnion(QuickUnion::new(n)),
            UnionFindType::WeightedQuickUnion => {
                UnionFindAlgs::WeightedQuickUnion(WeightedQuickUnion::new(n))
            }
        }
    }
}

#[enum_dispatch]
pub enum UnionFindAlgs {
    QuickFind,
    QuickUnion,
    WeightedQuickUnion,
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
