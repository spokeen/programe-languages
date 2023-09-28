fn main() {
    println!("Hello, world!");

    let s= String::from("hello");

    println!("{s}");

    let mut s2 = String::from("hello");
    
    s2.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s2); // 将打印 `hello, world!`
    
}
