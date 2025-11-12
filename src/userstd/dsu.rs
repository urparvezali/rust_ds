#![allow(dead_code)]
pub struct DSU {
    arr: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let arr = (0..n).collect();
        Self { arr }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.arr[x] != x {
            let root = self.find(self.arr[x]);
            self.arr[x] = root;
        }
        self.arr[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        self.arr[root_x] = root_y;
    }
}
