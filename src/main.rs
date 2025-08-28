fn main() {
    let s1: &str = "hello string";
    println!("s1: {}", s1);

    let mut s2: String = String::from("hello");
    s2.push_str(" world!");
    print!("s2 after push: {}", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{s3}, world!");
    println!("{s4}, world!");

    // 返回值与作用域

    test_vect_capacity();
}

fn test_vect_capacity() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // Len: 3, Capacity: 3
    v.push(4);
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // Len: 4, Capacity: 6
    v.push(5);
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // Len: 5, Capacity: 6
    v.push(6);
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // Len: 6, Capacity: 6
    v.push(7);
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // Len: 7, Capacity: 12
}
