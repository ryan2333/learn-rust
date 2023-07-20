fn main() {
    let a = [1, 2, 3, 4, 5];
    let sa = &a[1..3];
    assert_eq!(sa, &[2, 3])
}
