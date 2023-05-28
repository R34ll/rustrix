use std::fmt::{Debug, Display};

use super::Matrix;


impl<T:Clone+Copy+Default+Display> Debug for Matrix<T>{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = String::from("Matrix[\n");
        for row in 0..self.shape.0{
            tmp.push_str(" [");
            for col in 0..self.shape.1{
                tmp.push_str(format!(" {}",self.get((row,col))).as_str());
            }
            tmp.push_str(" ],\n");
        }
        tmp.push_str(format!("], Shape={:?}",self.shape).as_str());

        write!(f,"{}",tmp)
    }
} 





