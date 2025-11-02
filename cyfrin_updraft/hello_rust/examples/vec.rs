#![allow(unused)]

// Array - collection of elements with length known at compile time
// Slice - collection of elements with length not known at compile time 
 

fn main() {
 
     let mut v : Vec<i32> = Vec::new();
     v.push(1);
     v.push(2);
     v.push(3);
     println!("v: {:?}",v);
     
     let v : Vec<i8> = vec![1,2,3];
     let v = vec![1u8, 2, 3];
     
     let v : Vec<i8> = vec![0i8; 100];
     println!("v : {:?}", v);
     
     //Option<&i8>
     //Index valid => Some(&val)
     //Index invalid => None 
     println!("v : {}", v[1]);
     println!("v[1000]: {:?}", v.get(1000));
     
     // Update 
     let mut v : Vec<i8> = vec![1,2,3];
     v[0] = 99;
     
     // pop - remove last element 
     let mut v : Vec<i8> = vec![1,2,3];
     let x : Option<i8> = v.pop();
     println!("pop: {:?}", x);
     
     let x : Option<i8> = v.pop();
     println!("pop: {:?}", x);
     
     let x : Option<i8> = v.pop();
     println!("pop: {:?}", x);
     
     let x : Option<i8> = v.pop();
     println!("pop: {:?}", x);
     
     //Slice 
     let v = vec![1,2,3,4,5];
     let s = &v[1..4];
     println!("slice: {:?}", s);
     
}