use std::io;

fn main() {
    println!("Hello, world!");

    let num: u32 = get_num();
    println!("Num is {}", num);

    let fib: u32 = gen_fibonacci(num);
    println!("Fibonacci is {}", fib);
}


fn get_num() -> u32 {
    println!("Enter a positive integer (number) of the fibonacci number you need,\nstarting from the 1 (first)");

    loop {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        let num: u32 = match num.trim().parse() {
            Ok(num) => {
                if num == 0 { 
                    println!("Wrong input, input a positive integer, larger than 0!");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Wrong input, input a positive integer, larger than 0!");
                continue;
            },
        };

        // break the loop and return the value 
        println!("num {}", num);
        break num;
    }
}

fn gen_fibonacci(num: u32) -> u32 {
    // fibonacci numbers - each number is the sum of the two preceding ones.
    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, ...
    let mut first = 0;
    let mut second = 1;
   
    // count of generated fibonacci numbers
    let mut count = 2;
    while count < num {
        // We generate 2 numbers per 1 iteration
        first = first + second;
        second = second + first;
        // 2 fibonacci numbers
        count += 2;
    }
    // when choose from those numbers, first or second, according to the user num
    return if num % 2 == 0 { second } else { first };
}
