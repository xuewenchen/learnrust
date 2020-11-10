fn main() {
    // new
    let mut s1 = String::new();
    s1.push_str("this is s1");
    println!("s1 = {}", s1);


    // from
    let s2 = String::from("this is s2");
    println!("s2 = {}", s2);


    // string add
    let s3 = s1 +  " " + &s2;
    println!("s3 = {}", s3);

    // range string
    let ss = String::from("你是最棒的!");
    for s in ss.chars() {
        println!("s = {}", s)
    }


    
}
