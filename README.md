# rustrix
Matrix crate for rust

## Example
```rust
fn main(){
    let a = matrix![[1,2,3],[4,5,6]];
    let b = matrix![[7,8,9],[10,11,12]];

    println!("Result: {:?}",a.dot(&b.transpose()));
}
```
