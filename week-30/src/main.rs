fn main() {
    let v = vec![1, 2, 3];
    let v_iter = v.into_iter();
    let sum: i32 = v_iter.sum();
    println!("{}", sum);
    // try using the iterator again here.
}