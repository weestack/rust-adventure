fn reverseArray(a: &[i32]) -> Vec<i32> {
    a.iter().copied().rev().collect()
}
pub fn run() {
    let array = vec![1,2,3,4];
    let a = reverseArray(&array);
}