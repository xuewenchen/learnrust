fn main() {

    let mut temp: i32 = 0;
    
    let a: Option<i32> = Some(12);

    match a {
        Some(i) => {println!("i = {}", i)},
        None => {println!("nothing in it")},
    }

    match a {
        Some(i) => { temp = i },
        None => {println!("do nothing")},
    }

    println!("total num is = {}", temp);
}
