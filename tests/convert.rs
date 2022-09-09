macro_rules! convert_1d {
    ($v:expr, $to:ty) => {
        $v.iter().map(|x| *x as $to).collect::<Vec<_>>()
    };
}

macro_rules! convert_2d {
    ($v:expr, $to:ty) => {
        $v.iter()
            .map(|r| r.iter().map(|x| *x as $to).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
}

type NodeType = i32;

// non-direction
pub fn edges_to_g(n: usize, edges: &Vec<Vec<NodeType>>) -> Vec<Vec<NodeType>> {
    let mut g = vec![vec![]; n];
    for e in edges {
        g[e[0] as usize].push(e[1]);
        g[e[1] as usize].push(e[0]);
    }

    g
}

// directional
pub fn edges_to_dig(n: usize, edges: &Vec<Vec<NodeType>>) -> Vec<Vec<NodeType>> {
    let mut g = vec![vec![]; n];
    for e in edges {
        g[e[0] as usize].push(e[1]);
    }

    g
}

#[test]
fn test_convert() {
    let v1 = vec![1i32, 2, 3, 4, 5];
    let v2 = vec![1usize, 2, 3, 4, 5];
    assert_eq!(convert_1d!(v1, usize), v2);

    let v1 = vec![vec![1i32, 2, 3, 4, 5]];
    let v2 = vec![vec![1usize, 2, 3, 4, 5]];
    assert_eq!(convert_2d!(v1, usize), v2);

    let n = 6;
    let edges = vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]];
    let g = vec![vec![], vec![2], vec![1, 3, 4], vec![2], vec![2, 5], vec![4]];
    assert_eq!(edges_to_g(n, &edges), g);
}
