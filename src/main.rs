fn main() {
    let s1: &str = "hello string";
    println!("s1: {}", s1);

    let mut s2: String = String::from("hello");
    s2.push_str(" world!");
    print!("s2 after push: {}", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{s3}, world!");
    print!("s4: {}", s4);

    // 返回值与作用域
    let s5 = String::from("lake");
    let (s6, len) = calcute_length(s5);
    println!("The length of '{}' is {}.", s6, len);
    // struct
    struct_demo();
}

fn calcute_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}