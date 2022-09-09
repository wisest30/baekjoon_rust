use std::f64::consts::PI;
use std::io::Read;
pub fn get_tri_area(((y0, x0), (y1, x1), (y2, x2)): ((f64, f64), (f64, f64), (f64, f64))) -> f64 {
    (y0 * x1 + y1 * x2 + y2 * x0 - (y1 * x0 + y2 * x1 + y0 * x2)).abs() / 2.0
}

pub fn get_fen_area(r: f64, (x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> f64 {
    let theta = ((x0 * x1 + y0 * y1) / r / r).acos();

    r * r * theta / 2.0
}

pub fn get_common_area(rect: ((f64, f64), (f64, f64), (f64, f64), (f64, f64)), r: f64) -> f64 {
    let d0 = rect.0 .0 * rect.0 .0 + rect.0 .1 * rect.0 .1;
    let d1 = rect.1 .0 * rect.1 .0 + rect.1 .1 * rect.1 .1;
    let d2 = rect.2 .0 * rect.2 .0 + rect.2 .1 * rect.2 .1;
    let d3 = rect.3 .0 * rect.3 .0 + rect.3 .1 * rect.3 .1;
    let rect_len = rect.1 .1 - rect.0 .1;

    if d0 >= r * r {
        0.0
    } else if d2 <= r * r {
        rect_len * rect_len
    } else if d1 >= r * r && d3 >= r * r {
        let p0 = (rect.0 .0, (r * r - rect.0 .0 * rect.0 .0).sqrt());
        let p1 = ((r * r - rect.0 .1 * rect.0 .1).sqrt(), rect.0 .1);

        get_tri_area((rect.0, p0, p1)) + get_fen_area(r, p0, p1)
            - get_tri_area(((0.0, 0.0), p0, p1))
    } else if d1 < r * r && d3 >= r * r {
        let p0 = ((r * r - rect.1 .1 * rect.1 .1).sqrt(), rect.1 .1);
        let p1 = ((r * r - rect.0 .1 * rect.0 .1).sqrt(), rect.0 .1);

        get_tri_area((rect.0, p0, p1))
            + get_tri_area((rect.0, p0, rect.1))
            + get_fen_area(r, p0, p1)
            - get_tri_area(((0.0, 0.0), p0, p1))
    } else if d1 >= r * r && d3 < r * r {
        let p0 = (rect.0 .0, (r * r - rect.0 .0 * rect.0 .0).sqrt());
        let p1 = (rect.2 .0, (r * r - rect.2 .0 * rect.2 .0).sqrt());

        get_tri_area((rect.0, p0, p1))
            + get_tri_area((rect.0, p1, rect.3))
            + get_fen_area(r, p0, p1)
            - get_tri_area(((0.0, 0.0), p0, p1))
    } else {
        let p0 = ((r * r - rect.1 .1 * rect.1 .1).sqrt(), rect.1 .1);
        let p1 = (rect.2 .0, (r * r - rect.2 .0 * rect.2 .0).sqrt());

        get_tri_area((rect.0, p1, rect.3))
            + get_tri_area((rect.0, p0, p1))
            + get_tri_area((rect.0, rect.1, p0))
            + get_fen_area(r, p0, p1)
            - get_tri_area(((0.0, 0.0), p0, p1))
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let test_case = input.next().unwrap().parse::<usize>().unwrap();
    for tc in 1..=test_case {
        let f = input.next().unwrap().parse::<f64>().unwrap();
        let r = input.next().unwrap().parse::<f64>().unwrap();
        let t = input.next().unwrap().parse::<f64>().unwrap();
        let s = input.next().unwrap().parse::<f64>().unwrap();
        let g = input.next().unwrap().parse::<f64>().unwrap();

        let ans = if 2.0 * f >= g || r - t <= f {
            1.0
        } else {
            let mut v = vec![0.0];
            while v.last().unwrap() + g + 2.0 * s < r {
                v.push(v.last().unwrap() + g + 2.0 * s);
            }

            let mut rects = Vec::new();
            for &y in &v {
                for &x in &v {
                    rects.push((
                        (y + s + f, x + s + f),
                        (y + s + f, x + s + g - f),
                        (y + s + g - f, x + s + g - f),
                        (y + s + g - f, x + s + f),
                    ));
                }
            }

            let total_area = r * r * PI / 4.0;
            let mut sum_area = 0.0;

            for rect in rects {
                sum_area += get_common_area(rect, r - t - f);
            }

            1.0 - sum_area / total_area
        };

        println!("Case #{tc}: {:.15}", ans);
    }
}
