pub fn out(indent: u8, message: &str) {
    let mut msgs: Vec<&str> = vec![];
    for _ in 0..indent {
        msgs.push("\t");
    }
    msgs.push(&message);
    println!("{}", msgs.concat());
}