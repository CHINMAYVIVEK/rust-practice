fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let mut arr = vec![12, 11, 13, 5, 6];
    println!("Original Array: {:?}", arr);
    insertion_sort(&mut arr);
    println!("Insertion Sort: {:?}", arr);
}
