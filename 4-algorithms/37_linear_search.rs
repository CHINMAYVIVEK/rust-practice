fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &val) in arr.iter().enumerate() {
        if val == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let arr = vec![4, 2, 7, 1, 9];
    println!("Original Array: {:?}", arr);

    let target2 = 7;
    println!(
        "Linear Search: index of {} is {:?}",
        target2,
        linear_search(&arr, target2)
    );
}
