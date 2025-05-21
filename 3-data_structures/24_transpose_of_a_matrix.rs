fn transpose_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut transposed = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

fn main() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("Matrix m: {:?}", m);

    println!("Transpose: {:?}", transpose_matrix(&m));
}
