pub fn hello() -> String {
    "Hello Rust".to_string()
}

pub fn greet(name: &str) -> String {
    let mut s = String::from("Hello ");
    s.push_str(name);
    s 
}

pub fn append(mut s: String) -> String {
     s += "!";
     s
}
