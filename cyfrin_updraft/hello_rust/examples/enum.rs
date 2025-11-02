#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize{width: u32, height: u32}
}

fn main(){
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize{width:100, height:50};
    //println!("{}", cmd);
    
    //Debug
    println!("{:?}",cmd);

    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{}", cmd0 == cmd1);
    println!("{}", cmd0 == cmd0);
    
    // Option<T> = Some(T) | None
    let x : Option<i32> = Some(1);
    println!("1 :{:?}",x);

    let x : Option<i32> = None;
    println!("2: {:?}",x);

    // Result <T,E> = Ok(T) | Error
    // "100" -> 100
    let x : Result<i32, String> = Ok(100);
    println!("3: {:?}",x);

    // "123zxcv" -> error
    let x : Result<i32, String> = Err("Failed to parse string into number".to_string());

    println!("4: {:?}",x);
}