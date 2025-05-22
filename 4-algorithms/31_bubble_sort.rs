fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = vec![5, 3, 8, 4, 2];
    println!("Original Array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Bubble Sort: {:?}", arr);
}
