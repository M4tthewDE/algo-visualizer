pub struct QuickFindUF {
    pub ids: Vec<u64>,
}

impl QuickFindUF {
    pub fn new(n: u64) -> QuickFindUF {
        QuickFindUF {
            ids: (0..n).collect(),
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.ids[p];
        let qid = self.ids[q];

        // TODO: can this clone be removed?
        for (i, id) in self.ids.clone().iter().enumerate() {
            if *id == pid {
                self.ids[i] = qid;
            }
        }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.ids[p] == self.ids[q]
    }
}
