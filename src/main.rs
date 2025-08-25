fn main() {
    let s1: &str = "hello string";
    println!("s1: {}", s1);

    let mut s2: String = String::from("hello");
    s2.push_str(" world!");
    print!("s2 after push: {}", s2);


    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{s3}, world!");

    // 返回值与作用域

}


fn calcute_length(s: String) -> (String, usize){
    let length = s.len();
    (s,length)
}