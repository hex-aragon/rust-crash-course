#![allow(unused)]

fn main(){
    // let x : Option<i32> = Some(3);
    // let v : i32 = match x {
    //     Some(val) => val, 
    //     None => panic!("no value")
    // };


    // //Unwraps the inner value. Panics if None
    // let x : Option<i32> = None;
    // let i = x.unwrap();
    // println!("{}", i);



    let x : Result<i32, String> = Ok(3);
    let v : i32 = match x {
        Ok(val) => val, 
        Err(err) => panic!("err : {:?}", err)
    };

    // let x: Result<i32, String> = Err("somethinf failed".to_string());
    // let i = x.unwrap();
    // println!("result: {}", i);



    let x: Result<i32, String> = Err("something failed".to_string());
    let i = x.expect("something failed");


}