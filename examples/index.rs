#[macro_use]
use matrix_rs::Matrix;


fn main(){

    let a = Matrix![[4,5,6],[7,8,9],[1,2,3]];

    println!("{:?}",a);
    println!(">> {:?}",a.row(0));


}


