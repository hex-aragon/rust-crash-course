#![allow(unused)]

// Trait

struct Solidity {
    version: String, 
}

struct Vyper {
    version: String,
}


trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String {
        "Good luck".to_string()
    }
}

impl Compiler for Solidity{
     fn compile(&self, file_path: &str) -> String{
        format!("solc {}", file_path)
     }
}

impl Compiler for Vyper{
     fn compile(&self, file_path: &str) -> String{
        format!("vyper {}", file_path)
     }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main(){
    let sol = Solidity {version: "0.8".to_string() };
    let vy = Vyper {version: "0.4".to_string() };
    
    println!("sol help:{}",  sol.help());
    println!("vy help:{}",  vy.help());
    
    println!("sol compile:{}",  sol.compile("hello.sol"));
    println!("vy compile:{}",  vy.compile("hello.vy"));


    println!("sol compile:{}",  compile(&sol,"hello.sol"));
    println!("vy compile:{}",  compile(&vy, "hello.vy"));                                         
}