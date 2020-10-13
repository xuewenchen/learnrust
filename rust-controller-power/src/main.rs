fn main() {
    // on the stack
    let x = 1;
    let y = 1;
    {
        println!("x = {}", x);
        println!("y = {}", y);
    }
    println!("y = {}", y);

    // on the headp
    // move test, not work
    {
        let s1 = String::from("hello");
        println!("s1 = {}", s1);
        let s2 = s1;
        // println!("s1 = {}", s1); // s1 will not work, move
        println!("s2 = {}", s2);
    }

    // on the headp
    // move test, working using clone
    {
        let s1 = String::from("world");
        println!("s1 = {}", s1);
        let s2 = s1.clone();
        println!("s1 = {}", s1); // s1 will not work, move
        println!("s2 = {}", s2);
    }

    // copy trait
    // 常用类型都具有copy trait
    // i16...

    // move owner
    let s = String::from("move");
    let sx = test_owner(s);
    println!("sx = {}", sx);
}

fn test_owner(mystr: String) -> String {
    println!("mystr = {}", mystr);
    mystr
}
