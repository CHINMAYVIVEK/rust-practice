fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut merged = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut merged[..]);
    arr.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32], merged: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged[k] = left[i];
            i += 1;
        } else {
            merged[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        merged[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        merged[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Original Array: {:?}", arr);
    merge_sort(&mut arr);
    println!("Merge Sort: {:?}", arr);
}
