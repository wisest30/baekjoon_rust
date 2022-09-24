struct DisjointSet {
    g: Vec<i32>,
    g_set: std::collections::HashSet<i32>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        DisjointSet {
            g: (0..n as i32).collect::<Vec<_>>(),
            g_set: (0..n as i32).collect::<std::collections::HashSet<_>>(),
        }
    }

    pub fn group(&mut self, x: i32) -> i32 {
        if self.g[x as usize] == x {
            x
        } else {
            self.g[x as usize] = self.group(self.g[x as usize]);
            self.g[x as usize]
        }
    }

    pub fn merge(&mut self, x: i32, y: i32) -> bool {
        let gx = self.group(x);
        let gy = self.group(y);

        if gx == gy {
            false
        } else {
            self.g[gy as usize] = gx;
            self.g_set.remove(&gy);
            true
        }
    }

    pub fn get_group_set(&self) -> &std::collections::HashSet<i32> {
        &self.g_set
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
    assert!(!ds.merge(1, 2))
}
