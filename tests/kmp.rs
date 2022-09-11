pub fn get_pi<T>(s: &Vec<T>) -> Vec<usize>
where
    T: std::cmp::Eq,
{
    let n = s.len();
    let mut j = 0;
    let mut pi = vec![0; n];

    for i in 1..n {
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }

        if s[i] == s[j] {
            j += 1;
            pi[i] = j;
        }
    }

    pi
}

pub fn kmp<T>(s: &Vec<T>, p: &Vec<T>) -> Vec<usize>
where
    T: std::cmp::Eq,
{
    let mut ret = Vec::new();
    let n = s.len();
    let m = p.len();
    let mut j = 0;
    let pi = get_pi::<T>(p);
    for i in 0..n {
        while j > 0 && s[i] != p[j] {
            j = pi[j - 1];
        }
        if s[i] == p[j] {
            if j == m - 1 {
                ret.push(i + 1 - m);
                j = pi[j];
            } else {
                j += 1;
            }
        }
    }

    ret
}

#[test]
fn test_kmp() {
    let s = vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4];
    let p = vec![2, 3];
    assert_eq!(kmp::<i32>(&s, &p), vec![1, 7]);

    let s = vec![0, 0, 0, 0];
    let p = vec![0, 0];
    assert_eq!(kmp::<i32>(&s, &p), vec![0, 1, 2]);

    let s = vec![0, 0];
    let p = vec![0, 0, 0, 0];
    assert_eq!(kmp::<i32>(&s, &p), vec![]);
}
