use core::fmt::{Debug, Display};

use super::Matrix;


impl<T:Clone+Copy+Default+Display> Debug for Matrix<T>{

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Matrix[\n")?;
    
        for row in 0..self.shape.0 {
            f.write_str(" [")?;
    
            for col in 0..self.shape.1 {
                write!(f, " {}", self.get((row, col)))?;
            }
    
            f.write_str(" ],\n")?;
        }
    
        write!(f, "], Shape={:?}", self.shape)
    }
    
} 


// impl<T:Clone+Copy+Default+Display> Debug for Matrix<T>{

//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         let mut tmp = String::from("Matrix[\n");
//         for row in 0..self.shape.0{
//             tmp.push_str(" [");
//             for col in 0..self.shape.1{
//                 tmp.push_str(format!(" {}",self.get((row,col))).as_str());
//             }
//             tmp.push_str(" ],\n");
//         }
//         tmp.push_str(format!("], Shape={:?}",self.shape).as_str());

//         write!(f,"{}",tmp)
//     }
// } 





