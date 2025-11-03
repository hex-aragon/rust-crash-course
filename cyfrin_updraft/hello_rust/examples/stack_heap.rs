#![allow(unused)]


//Memory - stack and heap
// Stack
// - Stores data of fixed size known at compile time 
// - Fast 
// - LIFO (last in , first out)
// C 
// ---
// B 
// ---
// A


// Heap 
// - Stores data of unknown size at compile time 
// - Slower than stack
// - Memory safety is enforced through Rust's ownership and borrowsing rules

fn main() {

    //Stack 
    let x : i32 = 1; 
    let arr : [i32; 10] = [1; 10];
    
    //Heap 
    //문자열, 벡터 힙 저장 
    let mut s : String = "Hello".to_string();
    s += " world";
    
    let mut v = vec![];
    v.push(0);
    v.push(0);
    v.push(0);
    
    let  boxed = Box::new(1i32);
     
}