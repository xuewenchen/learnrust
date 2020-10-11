fn main() {
    // if
    let a = 2;
    if a == 1 {
        println!("a == 1");
    } else {
        println!("a != 1k");
    }
    println!("+++++++++++++++");

    // if else
    if a == 1 {
        println!("a == 1");
    } else if a == 2 {
        println!("a == 2");
    }
    println!("+++++++++++++++");

    // if let
    let condition = true;
    let x = if condition { 100 } else { 2 };
    println!("x = {}", x);
    println!("+++++++++++++++");

    // loop
    let mut counter = 0;
    loop {
        if counter == 10 {
            break;
        }
        println!("this is in loop");
        counter = counter + 1;
    }
    println!("+++++++++++++++");

    // while
    let mut i = 0;
    while i != 10 {
        println!("this is in while");
        i += 1;
    }
    println!("+++++++++++++++");

    // for
    let arr: [i32; 3] = [1, 2, 3];
    for item in arr.iter() {
        println!("item = {}", item);
    }
    println!("+++++++++++++++");
}
