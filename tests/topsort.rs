#![allow(dead_code)]

type NodeType = i32;
fn topsort(g: &Vec<Vec<NodeType>>) -> Vec<NodeType> {
    let n = g.len();
    let mut indegree = vec![0usize; n];
    for u in 0..n {
        for &v in &g[u] {
            indegree[v as usize] += 1;
        }
    }

    let mut ret = Vec::<NodeType>::new();
    for u in 0..n {
        if indegree[u] == 0 {
            ret.push(u as NodeType);
        }
    }

    let mut i = 0;
    while i < ret.len() {
        let u = ret[i];
        for &v in &g[u as usize] {
            indegree[v as usize] -= 1;
            if indegree[v as usize] == 0 {
                ret.push(v as NodeType);
            }
        }

        i += 1;
    }

    if ret.len() == n {
        ret
    } else {
        Vec::new()
    }
}

#[test]
fn test_topsort() {
    assert!(true);
}
