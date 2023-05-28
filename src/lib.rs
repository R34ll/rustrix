#![allow(dead_code)]
extern crate alloc;
use std::{ops::{Index, Add}, mem::transmute};

use alloc::vec::Vec;
mod math;
mod impl_methods;


#[macro_export]
macro_rules! matrix{
    ($($data:expr),*)=>{
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($data);)*
            Matrix::new_from_vec((temp_vec[0].len(),temp_vec.len()),temp_vec.into_iter().flatten().collect())
        }
    };
}




#[derive(Clone, PartialEq)]
pub struct Matrix<T>{
    shape:(usize,usize), // row, col
    pub data:Vec<T>
}

impl<T> Matrix<T>
where
    T:Clone+Default
{
    #[inline]
    pub fn new(shape:(usize,usize), data:&[T])->Self{
        assert_eq!(shape.0 * shape.1, data.len());
        Self{
            shape,
            data: data.to_vec()
        }
    }

    #[inline]
    pub fn new_from_vec(shape:(usize,usize),data:Vec<T>)->Self{
        Self{
            shape,
            data
        }
    }
    
    
    #[inline]
    pub fn zeros(shape:(usize,usize))->Self{
        Self{
            shape,
            data:vec![T::default();shape.0 * shape.1]
        }
    }
    
    pub fn get(&self,shape:(usize,usize))->T{
        // if shape.0 >= self.shape.1 || shape.1 >= self.shape.1{
        //     panic!("Index out of bounds.");
        // }

        self.data[shape.0 * self.shape.1 + shape.1].clone()
    }

    pub fn row(&self, row: usize) -> Matrix<T> {
        assert!(row <= self.shape.0, "{} - Index out of bounds.",row);

        let start = row * self.shape.1;
        let end = start + self.shape.1;

        let data = self.data[start..end].to_vec();

        Matrix::new_from_vec((self.shape.0,1), data)
    }


    pub fn transpose(&self)->Self{
        let mut transposed:Vec<T> = Vec::with_capacity(self.data.len());
        
        for c in 0..self.shape.0{
            for r in 0..self.shape.1{
                transposed.push(self.data[r * self.shape.0 + c].clone());
            }
        }

        Matrix{
            shape:(self.shape.1,self.shape.0),
            data:transposed
        }
    }

    pub fn shape(&self)->(usize,usize){
        self.shape
    }

    pub fn set_value(&mut self, shape:(usize,usize), value:T){
        //assert!(shape.0 < self.shape.0 || shape.1 < self.shape.1, "Invalid shape {:?}",shape);
        self.data[shape.0 * self.shape.1 + shape.1] = value;

    }

}




#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn transpose_func(){
        let data = matrix![[4,5,6],[6,7,8]];

        assert_eq!(
            matrix![[4,6],[5,7],[6,8]],
            data.transpose()
            );
    }

    #[test]
    fn get_row_func(){
        let data:Matrix<i32> = matrix!([4,5,6],[6,7,8]);

        assert_eq!(
            Matrix::new_from_vec((1,3),vec![6,7,8]), 
            data.row(1)
            );
    }

    #[test]
    fn get_func(){
        let data:Matrix<i32> = matrix!([4,5,6],[6,7,8]);
        assert_eq!(5, data.get((0,1)));
    }


    #[test]
    fn macro_test(){
        let data:Matrix<i32> = matrix!([4,5,6],[6,7,8]);

        assert_eq!(data.shape.0, 3);
        assert_eq!(data.shape.1, 2);
    }

    
}
