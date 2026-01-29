
fn main(){
  //declaring a variable 
 let mut x: i32 = 3 ; 
 let _y: i32 ;
 x+=2;
 assert_eq!(x,5);  //compares value 
 print!("success"); 

// mutable, shadowing and unmutable 

  let z = 6 ; 
  println!("the value of z before shadowing {}",z);
  let z= 7 ; 
  println!("the value of z after shadowing {}",z) ;
  // now here we will shadow and unmutable variable with a mutable variable 
  let mut z=2 ;
  
  while z<12 { z+=8 ; }
  println!("the value of z after shadowing by a mutable variable  is now {}",z);
  
}
  
