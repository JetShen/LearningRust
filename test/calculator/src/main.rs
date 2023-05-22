use std::io;


fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn div(a:i32 , b:i32) -> i32 {
    a / b
} 

fn mul(a:i32 , b:i32) -> i32 {
    a * b
}

fn restos(a:i32 , b:i32) -> i32 { // this mean mod but mod is a reserved word
    a % b
}

fn main() {
    loop{
        println!("Chose a number for A: ");
        //step to make an input and make the input a integer
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Error reading a");
        let a: i32 = match a.trim().parse() {  // i32 = integer 32 bits -128 to 127
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("Choose an option: ");
        println!("1. Add\n2. Sub\n3. Div\n4. Mul\n5. restos\n6. Exit"); // \n = new line
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error reading option");

        let option: u32 = match option.trim().parse() { // u32 = unsigned integer 32 bits 0 to 255
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Chose a number for B: ");
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("Error reading b");
        let b: i32 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You chose: {}", option);

        if option == 1 {
            println!("{} + {} = {}", a, b, add(a, b));
        }
        else if option == 2 {
            println!("{} - {} = {}", a, b, sub(a, b));
        }
        else if option == 3 {
            println!("{} / {} = {}", a, b, div(a, b));
        }
        else if option == 4 {
            println!("{} * {} = {}", a, b, mul(a, b));
        }
        else if option == 5 {
            println!("{} % {} = {}", a, b, restos(a, b));
        }
        else if option == 6 {
            break;
        }


    }
}
