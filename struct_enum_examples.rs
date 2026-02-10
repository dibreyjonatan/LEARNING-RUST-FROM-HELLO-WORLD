struct Rectangle{
    width : i32,
    height : i32,
}

impl Rectangle{
    fn area(&self) -> i32 {
        self.width*self.height 
    }
}

//### How to use it 

 fn main(){
  
    let rec=Rectangle{
        width : 2 ,
        height : 4,
    } ; 
    println!("aire : {}",rec.area()) ;
    println!("width : {} height : {} ", rec.width, rec.height ) ;
    

   enum Number{
     One,
     Two, 
     Three,
     }
     println!("{}",Number::One as i32 ) ; 
     println!("{}",Number::Two as i32);
     println!("{}",Number::Three as i32); 
     }
