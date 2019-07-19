fn main() {
    println!("Hello, world!");
    let x = 5;
    let y:i32 = 6;
    let z = another_function(x,y);
    println!("z is: {}", z);
    // let w = another_function(y,x);
    // println!("w is: {}", w);
}

fn another_function(x:i32, y:i32) -> i32 {
    return x + y
}
