use matrix_multiplication::{Matrix, multiply};

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Matrix after multiply {:?}", multiply(matrix, 3));
}