fn main() {

    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    give(s2);

    println!("s = {}", s1);
}

fn give(s1: & mut String) {
    println!("s = {}", s1);
}
