fn main() {
    // bool
    let mybool: bool = true;
    println!("bool = {}", mybool);

    // char
    let mychar: char = 'a';
    println!("char = {}", mychar);

    // number i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
    let mynumber: i8 = 10;
    println!("number = {}", mynumber);

    // 自适应类型, 和机器有关
    println!("size = {}", usize::max_value());

    // 数组
    // [Type; size]
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arrary[0] = {}", arr[0]);

    // 元祖
    let mytuple: (i8, f32, char) = (1, 1.2, 'h');
    println!("mytuple1 = {}", mytuple.0);
    println!("mytuple2 = {}", mytuple.1);
    println!("mytuple3 = {}", mytuple.2);
    // 元祖拆解
    let (x, y, z) = mytuple;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    // 数组传参
    show(arr);
}

fn show(arr: [u32; 5]) {
    for i in &arr {
        println!("{}", i);
    }
}
