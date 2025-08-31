## 栈与堆
栈与堆都是代码运行时可共使用的内存，但是结构不同。

### 栈
放入值的顺序储存值并以相反的顺序去除。栈中的所有数据都必须占用已知且固定的大小。在编译时大小未知或者大小可能变化的数据，要改为存在堆上。
栈 > last in, first out 
入栈 pushing onto the stack
出栈 poping off the stack

### 堆
堆是缺乏组织的：当向堆放入数据的时候，需要请求一定大小的空间。内存分配器 memory allocator在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置的 `指针` pointer。 这个过程称作 在堆上分配内存 allocating on the heap, 有时候 简称为 分配 allocating。

## 所有权规则

1、 Rust 中的每一个值都有一个所有者。
2、值在任意一时刻有且只有一个所有者。
3、当所有者离开作用域，这个值将被丢弃。

为什么是所有者离开作用域，而不是变量的原因
1、变量不一定是所有者
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 被移交所有权给s2。
```

2、所有权可以转移

### 变量


Rust 的所有权机制是其核心特性之一，通过编译时检查确保内存安全，无需运行时垃圾回收。以下是详细介绍及示例：

1. 所有权基础

核心规则：每个值有且仅有一个所有者，当所有者离开作用域时，值被自动释放（调用drop函数）。{
    let s = String::from("hello"); // s是所有者
    // 使用s...
} // s离开作用域，内存被释放

栈与堆的区别：
栈数据（如基本类型）：赋值时复制。
堆数据（如String）：赋值时转移所有权。
let s1 = String::from("hello");
let s2 = s1; // s1的所有权转移给s2，s1不再有效
println!("{}", s1); // 编译错误：s1已被转移



2. 所有权转移（Move）

堆数据的赋值或函数传递会转移所有权。fn take_ownership(s: String) {
    println!("Inside function: {}", s);
}

let s = String::from("hello");
take_ownership(s); // s的所有权转移到函数内部，外部无法再使用s

闭包中的move：强制闭包按值捕获变量，避免悬垂引用。let s = String::from("hello");
let closure = move || println!("{}", s); // s被闭包捕获，所有权转移



3. 借用（Borrowing）

不可变借用（&T）：允许临时访问数据，不转移所有权。fn calculate_length(s: &String) -> usize {
    s.len()
}

let s = String::from("hello");
let len = calculate_length(&s); // 借用s，不影响其所有权

可变借用（&mut T）：允许修改数据，但同一时间只能有一个可变引用。let mut s = String::from("hello");
{
    let mut_ref = &mut s;
    mut_ref.push_str(" world"); // 修改s
} // mut_ref离开作用域后，s可再次被借用

借用规则：
同一时间只能有一个可变引用，或多个不可变引用。
引用必须始终有效（不能引用已释放的内存）。




4. 切片（Slice）

字符串切片（&str）：引用字符串的一部分，不转移所有权。let s = String::from("hello world");
let word = &s[0..5]; // 切片从索引0到4（不包含5）
println!("First word: {}", word);

通用切片（&[T]）：适用于数组、向量等集合类型。let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3]; // 切片包含元素2和3



5. 所有权与并发

线程安全：所有权机制确保线程间无数据竞争。use std::thread;

let s = String::from("hello");
thread::spawn(move || {
    println!("Thread says: {}", s); // s的所有权转移到线程
});



通过所有权、借用和切片，Rust 在编译期保证内存安全，同时提供接近底层语言的性能。建议结合官方文档和实际项目深入理解。


所有权 (ownership)
 ├─ 转移 (move)   -> 变量赋值、函数传参
 ├─ 借用 (borrow)
 │    ├─ 不可变借用 (&T) -> 多个同时存在，不能修改
 │    └─ 可变借用 (&mut T) -> 同时只能有一个
 └─ 切片 (slice)  -> 对部分数据的引用，本质是借用
