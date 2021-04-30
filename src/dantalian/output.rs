pub fn out(indent: u8, message: &str) {
    let mut msgs: Vec<&str> = vec!["\t"; indent as usize];
    msgs.push(&message);
    println!("{}", msgs.concat());
}
