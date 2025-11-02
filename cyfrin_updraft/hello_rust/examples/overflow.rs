#![allow(unused)]


// Overflow doesn't panic when compiled with --release
fn main(){
    let mut x = u32::MAX;
    x += 1; 

    println!("u32 max: {}, x: {}", u32::MAX, x);
    

    // u32::checked_add - return one an overflow
    //let x = u32::checked_add(u32::MAX,1);
    let x = u32::checked_add(3,1);
    println!("checked_add: {:?}", x);
    // u32::wrapping_add - explicitly allow overflow



    let x = u32::wrapping_add(u32::MAX,1);
    println!("wrapping_add: {:?}", x);

}