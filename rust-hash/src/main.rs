use std::collections::HashMap;


fn main() {
    // init hashMap
    let mut my_hash = HashMap::new();
    my_hash.insert(12, 12);
    my_hash.insert(13, 12);

    // collection create
    let s1 = vec![1, 2, 3];
    let s2 = vec![String::from("chen"), String::from("xue"), String::from("wen")];
    let my_hash2: HashMap<_, _> = s1.iter().zip(s2.iter()).collect();

    // get value
    let k = 1;
    if let Some(v) = my_hash2.get(&k) {
        println!("v = {}", v);
    } else {
        println!("get nothing");
    }

    let k = 100;
    if let Some(v) = my_hash2.get(&k) {
        println!("v = {}", v);
    } else {
        println!("get nothing");
    }

    let v = my_hash2.get(&k);
    match v {
        Some(i) => { println!("i = {}", i);},
        None => { println!("nothing");},
    }

    // range value
    for(k, v) in &my_hash2 {
        println!("k = {}, v = {}", k, v);
    }

    // insert
    let mut my_hash_4 = HashMap::new();
    my_hash_4.insert(12, 12);
}
