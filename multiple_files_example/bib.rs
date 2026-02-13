// ici on écrit des fonctions avec pub afin que la fonction soit visible par d'autres fichiers rust

pub  fn addition(x: i32, y: i32 ) -> i32 {

     x + y 
} 

pub fn substract( x: i32, y: i32 ) -> i32{
    x-y 
}

pub fn multiply( x: i32 , y : i32 ) -> i32 {

   x*y   
}

pub fn divide( x: i32 , y : i32 ) -> f32 {

   (x/y) as f32    // convertir le resultat en un float32 en faisant une approximation
}

