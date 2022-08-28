pub fn upper_bound<T>(arr: &[T], val: T) -> usize
where
    T: Ord,
{
    if arr.is_empty() {
        0
    } else {
        let mut left = 0usize;
        let mut right = arr.len();
        while left + 1 < right {
            let m = (left + right) / 2;
            if arr[m] <= val {
                left = m;
            } else {
                right = m;
            }
        }

        assert!(left + 1 == right);
        if arr[left] <= val {
            left + 1
        } else {
            left
        }
    }
}

#[test]
fn test_upper_bound() {
    let arr = vec![0; 0];
    let res = upper_bound(&arr, 0);
    assert_eq!(res, 0);

    let arr = vec![1, 2, 3];
    let res = upper_bound(&arr, 100);
    assert_eq!(res, 3);

    let arr = vec![1, 2, 3];
    let res = upper_bound(&arr, 0);
    assert_eq!(res, 0);

    let arr = vec![0, 0];
    let res = upper_bound(&arr, 0);
    assert_eq!(res, 2);

    let arr = vec![1, 1, 1];
    let res = upper_bound(&arr, 1);
    assert_eq!(res, 3);

    let arr = [-3, -3, -1, -1, 1, 1];
    let res = upper_bound(&arr, -1);
    assert_eq!(res, 4);
}
