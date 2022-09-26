pub fn fun() {
    println!("{}:{}: ", file!(), line!());
}

pub mod bar2 {
    pub fn fun() {
        println!("{}:{}: ", file!(), line!());
    }
}
