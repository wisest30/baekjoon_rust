struct DisjointSet {
    g: Vec<i32>,
    g_set: std::collections::HashSet<i32>,
    cnts: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        DisjointSet {
            g: (0..n as i32).collect::<Vec<_>>(),
            g_set: (0..n as i32).collect::<std::collections::HashSet<_>>(),
            cnts: vec![1; n],
        }
    }

    pub fn group(&mut self, mut x: i32) -> i32 {
        while self.g[x as usize] != x {
            let nx = self.g[x as usize];
            self.g[x as usize] = self.g[nx as usize];
            x = nx;
        }

        x
    }

    pub fn merge(&mut self, x: i32, y: i32) -> bool {
        let gx = self.group(x);
        let gy = self.group(y);

        if gx == gy {
            false
        } else {
            self.g[gy as usize] = gx;
            self.g_set.remove(&gy);
            self.cnts[gx as usize] += self.cnts[gy as usize];
            true
        }
    }

    pub fn get_group_set(&self) -> &std::collections::HashSet<i32> {
        &self.g_set
    }

    pub fn cnt(&mut self, x: i32) -> usize {
        let g = self.group(x);
        self.cnts[g as usize]
    }
}

#[test]
fn test_disjoint_set() {
    let mut ds = DisjointSet::new(5);
    ds.merge(0, 3);
    assert_eq!(ds.group(0), ds.group(3));

    assert!(ds.merge(1, 2));
    assert_eq!(ds.group(1), ds.group(2));
    assert_ne!(ds.group(0), ds.group(1));
    assert_eq!(ds.get_group_set().len(), 3);
    assert!(!ds.merge(1, 2));

    assert_eq!(ds.cnt(0), 2);
    assert_eq!(ds.cnt(1), 2);
    assert_eq!(ds.cnt(2), 2);
    assert_eq!(ds.cnt(3), 2);
    assert_eq!(ds.cnt(4), 1);
}
