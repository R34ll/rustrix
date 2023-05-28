#[macro_use]
use rustrix::Matrix;


fn main(){

    let a = matrix![[4,5,6],[7,8,9],[1,2,3]];

    println!("{:?}",a);
    println!(">> {:?}",a.row(0));


}


