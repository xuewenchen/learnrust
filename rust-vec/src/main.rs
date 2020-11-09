fn main() {
    // new empty vector
    let my_vec: Vec<i32> = Vec::new();

    // new mut empty vector
    let mut my_vec2: Vec<i32> = Vec::new();
    my_vec2.push(1);
    my_vec2.push(2);
    my_vec2.push(3);
    


    // use 宏 init vector
    let my_vec3 = vec![1, 2, 3];


    let a: &i32 = &my_vec3[0];
    println!("a = {}", a);
    
    // get value
    match my_vec3.get(10) {
        Some(value) => { println!("a = {}", value); },
        _ => { println!("not in"); },
    }
    
    // loop1
    for i in &my_vec2 {
        println!("i = {}", i);
    }

    // loop2
    for i in &mut my_vec2 {
        *i += 1;
    }
    for i in &my_vec2 {
        println!("i = {}", i);
    }

    // enum vector
    enum Context {
        Text(String),
        Int(i32),
    }
    let my_vec4 = vec![
        Context::Text(String::from("hello")),
        Context::Int(32),
    ];
    for i in &my_vec4 {
        match i {
            Context::Text(v) => { println!("text = {}", v) },
            Context::Int(v) => { println!("int = {}", v) },
        }
    }
}
