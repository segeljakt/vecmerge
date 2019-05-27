
#[test]
fn simple() {
    use vecmerge::vecmerge;
    assert_eq!(vecmerge!([1,2,3] + [4,5] + [6]), vec![1,2,3,4,5,6]);

    let a = vec![1,2,3];
    let b = vec![6];

    assert_eq!(vecmerge!(a + [4,5] + b), vec![1,2,3,4,5,6]);

    assert_eq!(vecmerge!(vec![1,2,3] + vec![4,5,6]), vec![1,2,3,4,5,6]);
}
