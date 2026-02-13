//without pub it is a private function 
pub fn factorial ( x: i32 ) -> i32 {

    if x == 1 {
    
       x 
    }
    else {
    
      x*factorial(x-1) 
    }
}
