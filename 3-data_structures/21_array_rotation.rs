fn rotate_array<T: Clone>(arr: &mut Vec<T>, k: usize) {
    let n = arr.len();
    let k = k % n;
    arr.rotate_left(k);
}

fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    println!("Original Array: {:?}", arr);
    rotate_array(&mut arr, 2);
    println!("Rotated Array: {:?}", arr);
}
