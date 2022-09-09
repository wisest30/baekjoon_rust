pub fn get_tri_area(((y0, x0), (y1, x1), (y2, x2)): ((f64, f64), (f64, f64), (f64, f64))) -> f64 {
    (y0 * x1 + y1 * x2 + y2 * x0 - (y1 * x0 + y2 * x1 + y0 * x2)).abs() / 2.0
}

#[test]
fn test_get_tri_area() {
    const E: f64 = 0.0000001;

    let p0 = (0.0, 0.0);
    let p1 = (1.0, 0.0);
    let p2 = (0.0, 1.0);

    assert!((get_tri_area((p0, p1, p2)) - 0.5).abs() < E);

    let p0 = (10.0, 10.0);
    let p1 = (9.0, 10.0);
    let p2 = (10.0, 9.0);

    assert!((get_tri_area((p0, p1, p2)) - 0.5).abs() < E);

    let p0 = (10.0, 10.0);
    let p1 = (11.0, 10.0);
    let p2 = (10.5, 11.0);

    assert!((get_tri_area((p0, p1, p2)) - 0.5).abs() < E);
}
