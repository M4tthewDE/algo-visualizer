use enum_dispatch::enum_dispatch;

use self::{
    quick_find::QuickFind, quick_union::QuickUnion, weighted_quick_union::WeightedQuickUnion,
};

mod quick_find;
mod quick_union;
mod weighted_quick_union;

pub mod percolation;

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

    pub fn get_alg(&self, n: u64) -> UnionFindAlg {
        match self {
            UnionFindType::QuickFind => UnionFindAlg::QuickFind(QuickFind::new(n)),
            UnionFindType::QuickUnion => UnionFindAlg::QuickUnion(QuickUnion::new(n)),
            UnionFindType::WeightedQuickUnion => {
                UnionFindAlg::WeightedQuickUnion(WeightedQuickUnion::new(n))
            }
        }
    }
}

#[enum_dispatch]
pub enum UnionFindAlg {
    QuickFind,
    QuickUnion,
    WeightedQuickUnion,
}

#[enum_dispatch(UnionFindAlg)]
pub trait UnionFind {
    fn union(&mut self, p: usize, q: usize);
    fn connected(&self, p: usize, q: usize) -> bool;
    fn ids(&self) -> Vec<u64>;
}
