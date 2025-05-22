fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut low, mut high) = (0, arr.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    None
}

fn main() {
    let arr = vec![1, 3, 5, 7, 9];
    println!("Original Array: {:?}", arr);
    let target = 5;
    println!(
        "Binary Search: index of {} is {:?}",
        target,
        binary_search(&arr, target)
    );
}
