
fn main(){
 let  x = 2 ; 
 
{
 // x here is local and different from x outside the curly brackets because it is a local scope,
// the output of this program will be 0 in the scope and 2 outside the scope 
let  x=0 ; 
 println!("the value of x in the scope is {}",x) ; 

}

println!("the value of x outside the scope is {}",x);


}
