fn main() {
    let z: i32 = another_function(5);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    let y = {
        let x = 3;
        x + 1
    };
    y
}
