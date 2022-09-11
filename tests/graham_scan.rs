type T = i64;
pub fn graham_scan(points: &Vec<(T, T)>) -> Vec<(T, T)> {
    use std::collections::HashSet;

    fn angle((x0, y0): (T, T), (x1, y1): (T, T)) -> f64 {
        assert_ne!((x0, y0), (x1, y1));

        let x = (x1 - x0) as f64;
        let y = (y1 - y0) as f64;

        y.atan2(x)
    }

    fn ccw((x0, y0): (T, T), (x1, y1): (T, T)) -> i32 {
        let d = x0 * y1 - x1 * y0;
        if d == 0 {
            0
        } else if d > 0 {
            1
        } else {
            -1
        }
    }

    // make unique points
    let mut v = points
        .iter()
        .cloned()
        .collect::<HashSet<(T, T)>>()
        .into_iter()
        .collect::<Vec<_>>();

    if v.len() <= 2 {
        v.sort_unstable();
        return v;
    }

    let min_x_idx = v.iter().enumerate().map(|(i, v)| (v, i)).min().unwrap().1;
    v.swap(0, min_x_idx);

    let (x0, y0) = v[0];
    v[1..].sort_unstable_by(|(x1, y1), (x2, y2)| {
        let a1 = angle((x0, y0), (*x1, *y1));
        let a2 = angle((x0, y0), (*x2, *y2));

        if a1 == a2 {
            let d1 = (x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0);
            let d2 = (x2 - x0) * (x2 - x0) + (y2 - y0) * (y2 - y0);
            d1.cmp(&d2)
        } else {
            a1.partial_cmp(&a2).unwrap()
        }
    });

    let mut ret = vec![v[0], v[1]];
    for i in 2..v.len() {
        let mut last = ret.len() - 1;
        let mut llast = last - 1;

        while ret.len() >= 2
            && ccw(
                (ret[last].0 - ret[llast].0, ret[last].1 - ret[llast].1),
                (v[i].0 - ret[last].0, v[i].1 - ret[last].1),
            ) <= 0
        {
            ret.pop().unwrap();
            if ret.len() < 2 {
                break;
            }

            last -= 1;
            llast -= 1;
        }

        ret.push(v[i]);
    }

    ret
}

#[test]
fn test_graham_scan() {
    let points = vec![
        (3, 2),
        (1, 1),
        (2, 1),
        (1, 2),
        (1, 3),
        (2, 3),
        (2, 2),
        (3, 1),
    ];

    let ret = graham_scan(&points);
    assert_eq!(ret, vec![(1, 1), (3, 1), (3, 2), (2, 3), (1, 3)]);
}
