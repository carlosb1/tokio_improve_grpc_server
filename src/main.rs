extern crate mvpgrpc;
extern crate futures;


use std::net::SocketAddr;

use futures::Future;
use futures::Stream;
use mvpgrpc::{ex_simple, ex_1};


struct MyGreeter;

impl MyGreeter {
    fn new() -> MyGreeter {
        MyGreeter
    }
}


#[mvpgrpc::ex_0]
pub fn ex0() -> i32 {
    0
}

#[mvpgrpc::ex_1(ex_1a)]
pub fn ex1() -> i32 {
    1
}


fn main() {
    println!("{:}",ex0());
    println!("{:}",ex1());
    println!("{:}",ex_1a());
    //println!("{:}",my_new_answer());
    //let greeter = MyGreeter::new();

}
