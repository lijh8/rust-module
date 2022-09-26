fn main() {
    //inline
    fun();
    main2::fun();

    //lib.rs: foo
    hello_lib::foo::fun();
    hello_lib::foo::foo2::fun();
    hello_lib::foo::bar::fun();
    hello_lib::foo::bar::bar2::fun();
}

fn fun() {
    println!("{}:{}: ", file!(), line!());
}

pub mod main2 {
    pub fn fun() {
        println!("{}:{}: ", file!(), line!());
        super::fun();
    }
}
