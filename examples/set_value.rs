
#[macro_use]
use rustrix::*;


fn main(){
    let mut a = matrix![[4,5,6,7],[1,2,3,4],[9,5,3,2],[12,45,7,0]];

    
    println!("{:?}",a);





    a.set_value((3,3), 90);
    println!("{:?}",a);




}

