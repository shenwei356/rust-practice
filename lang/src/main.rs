fn main() {
    let s: String = "hello".to_string();
    let f = || {
        println!("{:?}", s);
    };
    f();
    f();

    println!("{:?}", s);
}
