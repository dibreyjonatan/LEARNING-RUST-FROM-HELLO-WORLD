
fn main(){

 let x= String :: from("testing it\n");
 let y=x.clone() ;
 println!("x : {} y : {}",x,y);


let mut z: i32=4 ;
let y= &mut z ;
 *y=6 ;
println!(" y: {} ",*y);

//several immutable borrow
let a : i32=3 ;
let b: &i32=&a ;
let c: &i32=&a;
println!("a : {} b: {} c:{} ", a , b,c);

}
