
fn test_fn() -> i32 {

  2 

}

fn test_unit(){
println!(" testing unit type") ;
}
fn main(){

  let x : i32 = test_fn() ;

  println!("X is {}",x) ; 

  let y : ()=test_unit() ;
  assert!(y==()) ; 
  println!("you  have successfully understood unit types"); 

}
