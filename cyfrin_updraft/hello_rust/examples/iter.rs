#![allow(unused)]

use std::collections::HashMap;

//Iterators 
// - map, filter, collect
fn main(){
    let vals: Vec<u32> = vec![1,2,3,4,5];
    //into_iter - iterate T - 각 항목이 T인 타입 반복자 출력, 소유권이 반복자로 이전 
    //iter - iterate &T 각 반복에서 반복되는 값에 대한 참조를 반환하는 반복자 생성 
    //iter_mut - iterate &mut T 각 반복마다 가변참조를 제공하는 반복자 생성
    

    // for v : &mut u32  in vals.iter_mut() {
    //     //

    // }

    // for v in vals.iter_mut() {
    //     // 
    // }

    // &u32 
    // map  - f(x: &T)
    let v2: Vec<u32> =  vals.iter().map(|x: &u32| x + 1).collect();
    println!("v2 {:?}", v2);

    // collect
    let vals: Vec<(&str, u32)> = vec![("a",1), ("b",2), ("c", 3)];
    let v: Vec<(String, u32)> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("vec {:?}",v);

    let v: HashMap<String , u32> = vals.iter().map(|v| (v.0.to_string(), v.1+1)).collect();
    println!("hash map {:?}", v);

    //chaining filter and map
    let vals : Vec<u32> = vec![1,2,3,4,5];

    //&T
    let v: Vec<u32> = vals 
        .iter()
        .filter(|x: &&u32| **x <= 3)
        .map(|x: &u32| x + 1).collect();
    println!("filter -> map  {:?}", v);



    // T
    let v: Vec<u32> = vals 
        .into_iter()
        .filter(|x: &u32| *x <= 3)
        .map(|x: u32| x + 1).collect();
    println!("filter -> map  {:?}", v);


}