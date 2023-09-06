use std::collections::HashMap;

use rand::seq::SliceRandom;

use super::{UnionFind, UnionFindAlg, UnionFindType};

pub fn simulate(iterations: usize, dimension: usize, uf_type: UnionFindType) -> f64 {
    let mut ratios = Vec::new();

    for _ in 0..iterations {
        let mut p = Percolation::new(dimension, uf_type.clone());
        while !p.percolates() {
            p.open_random_site();
        }

        ratios.push(p.open_site_count() as f64 / (dimension * dimension) as f64);
    }

    let mut sum = 0.0;
    for ratio in ratios {
        sum += ratio;
    }

    sum / iterations as f64
}
#[derive(Clone)]
enum Tile {
    Free,
    Blocked,
}

struct Percolation {
    n: usize,
    grid: Vec<Tile>,
    blocked_indeces: HashMap<usize, u64>,
    uf: UnionFindAlg,
}

impl Percolation {
    pub fn new(n: usize, uf_type: UnionFindType) -> Percolation {
        let mut blocked_indeces = HashMap::new();
        let grid = vec![Tile::Blocked; n * n];

        for i in 0..(n * n) {
            blocked_indeces.insert(i, 1);
        }

        let mut uf = uf_type.get_alg((n * n + 2) as u64);
        for i in 0..n {
            uf.union(i, n * n);
            uf.union(i + n * n - n - 1, n * n + 1);
        }

        Percolation {
            n,
            grid,
            blocked_indeces,
            uf,
        }
    }

    fn open_at_index(&mut self, id: usize) {
        self.grid[id] = Tile::Free;

        if id < self.n * self.n - 1 && matches!(self.grid[id + 1], Tile::Free) {
            self.uf.union(id, id + 1);
        }

        if id > 0 && matches!(self.grid[id - 1], Tile::Free) {
            self.uf.union(id, id - 1);
        }

        if id < self.n * self.n - self.n - 1 && matches!(self.grid[id + self.n], Tile::Free) {
            self.uf.union(id, id + self.n);
        }
        if id >= self.n && matches!(self.grid[id - self.n], Tile::Free) {
            self.uf.union(id, id - self.n);
        }
    }

    fn open_random_site(&mut self) {
        let keys = self.blocked_indeces.keys().collect::<Vec<&usize>>();
        let i = keys.choose(&mut rand::thread_rng()).unwrap();

        self.open_at_index(**i);
    }

    fn open_site_count(&self) -> u64 {
        let mut n = 0;
        for i in &self.grid {
            if matches!(i, Tile::Free) {
                n += 1;
            }
        }

        n
    }

    fn percolates(&self) -> bool {
        self.uf.connected(self.n * self.n, self.n * self.n + 1)
    }
}
