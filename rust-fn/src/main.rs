// not has paramter
fn echo() {
    println!("hello world");
}

// paramter must has type
fn echo_1(a: i32) {
    println!("a = {}", a);
}

// has return has;
// fn echo_2(m: i32, n: i32) -> i32 {
//     return m + n;
// }

// has return not has;
fn echo_3(m: i32, n: i32) -> i32 {
    m + n
}

fn main() {
    // not paramter
    echo();

    // has paramter
    let a = 1;
    echo_1(a);

    // has return
    let r = echo_3(1, 2);
    println!("return = {}", r);

    // 语句
    // let y = 1;
    // let x = (let y = 1); // will error 这个是语句，是不会返回值的
    // println!("x = {}", x);

    // will not error 这是一个表达式，是会返回值的
    let x = {
        let y = 1;
        y + 1
    };
    println!("x = {}", x);
}
