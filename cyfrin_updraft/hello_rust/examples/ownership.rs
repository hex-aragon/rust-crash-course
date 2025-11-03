#![allow(unused)]

// Ownership rules 
// 1. Each value has an owner 
// 2. There can only be one owner at a time 
// 3. When the owner goes out of scope, the value will be dropped 

fn take(s: String){
    println!("take {s}");
    //s is dropped
}

fn copy(v: i32) {
    println!("copy {v}");
    // v is dropped here 
}

fn main() {

  // 1. Each value has an owner 
  // Owner of s is s 
  let s = String::from("rust");
  // Owner of i is i 
  let i = 1; 
  
  // 2. There can only be one owner at a time 
  let s = String::from("dog");
  // Owner of s is s1 
  let s1 = s; 
  // Owner of s is s2 
  let s2 =s1; 
  println!("{s2}");
  
  // 3. When the owner goes out of scope, the value will be dropped 
  //println!("{s}");
  let s = String::from("cat");
  {
    //let s1 = s; 
    //s is dropped

  }//scope ends here
  println!("{s}");
  take(s);

  // Ownership doesn't move for
  // types that implement the Copy
  // trait

  //Owner of i is i
  let i  = 1; 
  //Owner of i1 is i1 
  let i1 = i; 

}