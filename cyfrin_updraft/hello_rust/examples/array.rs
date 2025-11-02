#![allow(unused)]

// Array - collection of elements with length known at compile time
// Slice - collection of elements with length not known at compile time 
 

fn main() {
 
        
    // Array 
    let arr: [u32; 3] = [1,2,3];
    println!("arr[0] : {}", arr[0]);
    
    // Write 
    let mut arr: [u32; 3] = [1,2,3];
    arr[1] = 99;
    
    let arr: [u32; 10] = [0; 10];
    println!("arr: {:?}", arr);
    
    
    // Slice
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
    println!("nums: {:?}", nums);
    
    // First 3 
    let s: &[i32] = &nums[..3];
    println!("s: {:?}", s);
    
    // Last 3 (indexes = 7, 8, 9)
    let s : &[i32] = &nums[7..];
    println!("s: {:?}", s);
    
    let s : &[i32] = &nums[3..7];
    println!("s: {:?}", s);
    
}