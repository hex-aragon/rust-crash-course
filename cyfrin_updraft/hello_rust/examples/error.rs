#![allow(unused)]

#[derive(Debug)]
enum MatchError {
    DivByZero,
    Other 
}


// Error handling 
// panic!
// Option<T> = Some(T) | None
// Result<T, E> = Ok(T) | Err(E)

fn main(){
   // panic!("something went wrong");



    let v = vec![1,2,3];
    // Index out of bounds 
    // v[99];

    //let x : Option<i32> = v.get(1)
    let x : Option<i32> = v.get(1).copied();
    match x {
        Some(val) => println!("x: {:?}", val),
        None => println!("x: none")
    }

    // Result<T, E> = Ok(T) | Err(E)
    let x = 1;
    let y = 0; 
    //This will panic. Division by 0
    //let q = x / y ;

    let q : Result<i32, MatchError> = if y != 0 {
        Ok(x / y)
    } else {
       // Err("div by 0".to_string())
       Err(MatchError::DivByZero)
    };

    match q {
        Ok(val) => println!("x / y = {:?}", val),
        Err(err) => println!("x / y error {:?}",err)
    }

}