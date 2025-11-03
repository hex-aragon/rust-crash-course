#![allow(unused)]

fn take(s : String) {
    println!("take {s}");
}

fn borrow(s: &String){
    println!("borrow {s}");
}

fn main() {
    // Take Ownership
    let s  = String::from("rust");
    borrow(&s);
    println!("{s}");
    //take(s);
    // s is dropped after take(s)
    // This will not compile
    //println!("{s}");


    // - Creates a reference (either mutable or immutable)
    // Immutable borrow 
    let s = String::from("rust");
    //let s1 = &s; 
    // Owner of i is i 
    let i = 1; 

    // 2. There can only be one owner at a time
    let s  = String::from("dog");
    let s1 = &s;
    let s2 = &s; 
    let s3 = s2;

    // Mutable borrow 
    let mut s = String::from("rust");
    let s1 = &mut s; 

    // s1 has read and write access to s
    s1.push_str("t");
    let s2 = &mut s;
    s2.push_str("a");

    // - Doesn't move ownership
    // - Immutable reference - any number of read-only access to a value
    
    // - Mutable reference - only one read and write access to a value at a time
    // - Either immutable or mutable borrow, but not both simultaneously.
    let mut s = String::from("rust");
    
    // s1, s2 and s3 have read-only access to s 
    let s1 = &s;
    let s2 = &s; 
    //let s3 = &mut s; 
    println!("s1: {s1}");
    //s3.push_str("t");

    // - Reference must not outlive the value
    let s = String::from("rust");
    let s1 = &s;
    {
        let s2 = s;
    }
    //println!("s1: {s1}");
    // s2 and s no longer exist
    // s1 references s 



}