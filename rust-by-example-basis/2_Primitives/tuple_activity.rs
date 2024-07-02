/*
任务:
1. 对 Matrix 功能添加打印
2. 构造一个新的函数 matrix 和 transpose(matrix), 输出实现结果
纠错说明，对于元组结构体（没有声明成员名称）
其打印格式并不是 Matrix{; ;}, 而是 Matrix(,..,)
*/
struct Matrix(f32, f32, f32, f32);

use std::fmt::{self, Formatter, Display};

impl Display for Matrix{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} {})\n", self.0, self.1)?;
        write!(f, "({} {})", self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let transpose_matrix = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    transpose_matrix
}

fn main(){
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}