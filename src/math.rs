#![macro_use]
use super::Matrix;


use core::ops::{
    Add,Sub, Mul, Div
};


impl<T:Add<Output=T> + Clone+Copy+Default> Add<Matrix<T>> for Matrix<T>
{
    type Output = Matrix<T>;

    fn add(self, other:Self) -> Self{
        assert!(self.shape == other.shape,"Incompatible matrix dimensions for addition");
        let mut result = Matrix::zeros(self.shape);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                result.set_value((i, j), self.get((i, j)) + other.get((i, j)));
            }
        }
        result
    }
}


impl<T:Sub<Output=T> + Default+ Clone+Copy> Sub<Matrix<T>> for Matrix<T>{
    type Output = Matrix<T>;

    fn sub(self, other:Self) -> Self{
        assert!(self.shape == other.shape,"Incompatible matrix dimensions for addition");
        let mut result = Matrix::zeros(self.shape);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                result.set_value((i, j), self.get((i, j)) - other.get((i, j)));
            }
        }
        result
    }
}

impl<T:Mul<Output=T> + Clone+Default+Copy> Mul<Matrix<T>> for Matrix<T>{
    type Output = Matrix<T>;

    fn mul(self, other:Self) -> Self{
        assert!(self.shape == other.shape,"Incompatible matrix dimensions for addition");
        let mut result = Matrix::zeros(self.shape);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                result.set_value((i, j), self.get((i, j)) * other.get((i, j)));
            }
        }
        result
    }
}

impl<T:Div<Output=T> + Clone+Copy+Default> Div<Matrix<T>> for Matrix<T>{
    type Output = Matrix<T>;

    fn div(self, other:Self) -> Self{
        assert!(self.shape == other.shape,"Incompatible matrix dimensions for addition");
        let mut result = Matrix::zeros(self.shape);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                result.set_value((i, j), self.get((i, j)) / other.get((i, j)));
            }
        }
        result
    }
}



impl<T> Matrix<T>
where 
    T: std::ops::Mul<Output = T> + std::ops::AddAssign + Clone + Default,
{
    pub fn dot(&self, other: &Matrix<T>) -> Matrix<T> {
        let (self_rows, self_cols) = self.shape;
        let (other_rows, other_cols) = other.shape;

        // Check if the matrices can be multiplied
        if self_cols != other_rows {
            panic!("Matrix dimensions are incompatible for dot product.");
        }

        let mut result_data = Vec::with_capacity(self_rows * other_cols);

        for i in 0..self_rows {
            for j in 0..other_cols {
                let mut sum: T = Default::default();
                for k in 0..self_cols {
                    sum += self.get((i, k)) * other.get((k, j));
                }
                result_data.push(sum);
            }
        }

        Matrix {
            shape: (self_rows, other_cols),
            data: result_data,
        }
    }
}








#[cfg(test)]
mod math{
    #![macro_use]
    use crate::*;

    #[test]
    fn dot(){
        let a:Matrix<i32> = matrix![[4,5,6],[7,8,9]];
        
        assert_eq!(a.clone().dot(&a.transpose()), matrix![[100,75],[163,117]]);
    }

    #[test]
    fn addition(){
        let a:Matrix<i32> = matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = matrix![[9,4,8],[1,4,6]];

        assert_eq!(a+b, matrix![[13,9,14],[6,8,9]]);
    }

    #[test]
    fn subtraction(){
        let a:Matrix<i32> = matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = matrix![[9,4,8],[1,4,6]];

        assert_eq!(a-b, matrix![[-5,1,-2 ],[4,0,-3]]);
    }

    #[test]
    fn multiplication(){
        let a:Matrix<i32> = matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = matrix![[9,4,8],[1,4,6]];

        assert_eq!(a*b, matrix![[36,20,48 ],[5,16,18]]);
    }

     #[test]
    fn division(){
        let a:Matrix<i32> = matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = matrix![[9,4,8],[1,4,6]];

        assert_eq!(a/b, matrix![[0,1,0 ],[5,1,0]]);
    }




}






