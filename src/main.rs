extern crate mvpgrpc;
extern crate futures;


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


#[mvpgrpc::ex_2(param1="my_first_param!")]
pub fn ex2() {
}

fn main() {
    println!("{:}",ex0());
    println!("{:}",ex1());
    //println!("{:}",ex_1a());
    ex2();
    //println!("{:}",my_new_answer());
    //let greeter = MyGreeter::new();

}
