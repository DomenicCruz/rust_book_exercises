use std::io;

fn main() {
    //[°F] = [°C] × 9⁄5 + 32 	[°C] = ([°F] − 32) × 5⁄9
    println!("Hello, world!");

    println!("C  {}", cvert_c_to_f(10.0));
    println!("C  {}", cvert_c_to_f(22.0));

    println!("Celsius-Farenheit converter");
    
    loop {
        println!("Input a number to convert");
        
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
       
        let num: f64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Type 1 to convert °C -> °F, or 0 for the °F => °C");
        loop {
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: i32 = match choice.trim().parse(){
                Ok(choice) => choice,
                Err(_) => {
                    println!("this was not a number, enter 1 or 0");
                    continue;
                },
            };

            if choice == 1 {
                println!("Your answer is {} °C ", cvert_c_to_f(num));
                break;
            }else if choice == 0 {
                println!("Your answer is {} °F ", cvert_f_to_c(num));
                break;
            }else {
                println!("enter 1 or 0");
                continue;
            }
        }
    }
}


fn cvert_c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn cvert_f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}
