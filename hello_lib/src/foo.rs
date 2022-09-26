pub mod bar;

pub fn fun() {
    println!("{}:{}: ", file!(), line!());
}

pub mod foo2 {
    pub fn fun() {
        println!("{}:{}: ", file!(), line!());
    }
}
