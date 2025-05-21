fn add_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter()
        .zip(b.iter())
        .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect())
        .collect()
}

fn main() {
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];

    println!("Matrix a: {:?}", a);
    println!("Matrix b: {:?}", b);

    println!("Matrix Addition: {:?}", add_matrices(&a, &b));
}
