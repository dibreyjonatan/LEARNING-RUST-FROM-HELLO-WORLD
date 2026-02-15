// une fonction closure est l'equivalent de fonction lambda en python.
// c'est une fonction annonyme qui peut etre utilisé par une autre fonction.
// pour utiliser une fonction closure on utilise  |a,b,...| pour mettre les arguements de la fonction
// suivi par l'implementation de la fonction 

fn sum(x: i32, y: i32 ) -> i32 {
    x+y
}
fn test_pair ( x: i32 ) -> bool {
  match x {
     0 => true ,
     1 => false, 
     _ => false,  // tout autre cas est representé par _
  }
}
fn main(){
    let somme= |x,y| x+y ;
    println!("la somme de 2 et 3 est : {}", sum(2,3)) ;
    println!("la somme de 2 et 3 est : {}", somme(2,3)) ;  
    
    //code de test de parité en utilisant une fonction de closure
    let parity= |x| x%2 ; //fait le calcul et retourne 0 ou 1, et c'est 
    println!(" 2 est pair : {}", test_pair(parity(2)));
    println!(" 3 est pair : {}", test_pair(parity(3)));
}
