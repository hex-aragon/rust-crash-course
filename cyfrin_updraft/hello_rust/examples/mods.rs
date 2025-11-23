use hello_rust::{my, foo};


fn main(){
    my::print();
    my::a::print();


    let s = my::a::build(1);

    my::a::print_foo();
}