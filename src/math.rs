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
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Default + Clone + Copy 
{
    pub fn dot(&self, other:&Matrix<T>)->Matrix<T>{
        // assert!(self.shape.1 != other.shape.0,"Incompatible matrix dimensions for dot product");
        
        let mut result = Matrix::zeros((self.shape.0, other.shape.1));
        dbg!(result.shape, result.data.len());
        
        for i in 0..self.shape.0{
            for j in 0..other.shape.1{
                let mut dot_product = T::default();
                for k in 0..self.shape.1{
                    let aik = self.data[k*self.shape.1+k];
                    let akj = other.data[k*other.shape.1+j.clone()];
                    dot_product = dot_product + (aik * akj);
                } 
                result.set_value((i,j), dot_product);
            } 
        }

        result


    }
}








#[cfg(test)]
mod math{
    use super::Matrix;

    #[test]
    fn dot(){
        let a:Matrix<i32> = Matrix![[4,5,6],[7,8,9]];
        
        assert_eq!(a.clone().dot(&a.transpose()), Matrix![[100,75],[163,117]]);
    }

    #[test]
    fn addition(){
        let a:Matrix<i32> = Matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = Matrix![[9,4,8],[1,4,6]];

        assert_eq!(a+b, Matrix![[13,9,14],[6,8,9]]);
    }

    #[test]
    fn subtraction(){
        let a:Matrix<i32> = Matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = Matrix![[9,4,8],[1,4,6]];

        assert_eq!(a-b, Matrix![[-5,1,-2 ],[4,0,-3]]);
    }

    #[test]
    fn multiplication(){
        let a:Matrix<i32> = Matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = Matrix![[9,4,8],[1,4,6]];

        assert_eq!(a*b, Matrix![[36,20,48 ],[5,16,18]]);
    }

     #[test]
    fn division(){
        let a:Matrix<i32> = Matrix![[4,5,6],[5,4,3]];
        let b:Matrix<i32> = Matrix![[9,4,8],[1,4,6]];

        assert_eq!(a/b, Matrix![[0,1,0 ],[5,1,0]]);
    }




}






