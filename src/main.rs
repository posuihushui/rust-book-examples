mod guss_game;

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

    use_after_free_prevented();
    // call the guessing game from the separate module
    guss_game::run();
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

// 结构体
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_demo() {
    println!("struct demo-------------->");
    // 栈上的struct
    let p1 = Point { x: 10, y: 20 };
    println!("p1: {:?}", p1);
    println!("p1.x: {}", p1.x);
    println!("p1.y: {}", p1.y);

    // 对上的struct
    let p2 = Box::new(Point { x: 30, y: 40 });
    println!("p2: {:?}", p2);
    println!("p2.x: {}", p2.x);
    println!("p2.y: {}", p2.y);

    // 可变的struct
    let mut p3 = Point { x: 50, y: 60 };
    p3.x = 70;
    p3.y = 80;
    println!("p3: {:?}", p3);

    // 计算面积
    let length = 10;
    let width = 20;
    let area1 = area(length, width);
    println!("The area of rectangle is {}.", area1);

    // display
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("rect1 is {rect1:?}");
    test1();
    test_scope();
    test_define();
    test_descructing();
    test_type();
    test_add_number();
    test_vector();
    test_loop1();
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn test1() {
    let x: i32 = 5;
    assert_eq!(x, 5);
}

fn test_scope() {
    let x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("x value is {}", x);
    // println!("The value of x is {} and value of y is {}", x, y)
}

fn test_define() {
    let x: &str = "hello";
    println!("{}, world", x);
}

fn test_descructing() {
    let (mut x, y, z) = (1, 2, 3);
    x += 1;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn test_type() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    println!("x is of type {}", type_of(&x));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn test_add_number() {
    let v1 = 251_u16 + 100;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("v1: {}, v2: {}", v1, v2);
}
fn test_vector() {
    let v = vec![1, 2, 3, 4];
    println!("v: {:?}", v);
    println!("len: {}, capacity: {}", v.len(), v.capacity());

    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // 输出: Len: 3, Capacity: 3
    v.push(4); // 触发扩容
    println!("Len: {}, Capacity: {}", v.len(), v.capacity()); // 输出: Len: 4, Capacity: 6
}
fn test_loop1() {
    let mut sum: i32 = 0;
    for i in -3..1 {
        sum += i;
        println!("i is {} , sum current is {}", i, sum);
    }
    for i in -3..=1 {
        println!("i is {}", i);
    }
    for c in 'a'..='e' {
        println!("c is {}", c);
    }
}
fn use_after_free_prevented() {
    let s = String::from("hello");
    let r = &s;
    let _s2 = s.clone(); // clone instead of moving while borrowed
    println!("r: {}", r);
}
