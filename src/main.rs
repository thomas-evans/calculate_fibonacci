use std::io;

fn calculate_fibonacci(n: u32) -> u32{
    let mut fibonacci_number: Vec<u32> = vec![0,1];
    let mut index: u32 = 0;
    while index  < n{
        let next_number = fibonacci_number[0] + fibonacci_number[1];
        fibonacci_number.remove(0);
        fibonacci_number.push(next_number);
        index += 1;
    }
    fibonacci_number[0]
}

fn user_prompt(fibonacci_calculator: fn(u32) -> u32) -> u32{
    loop{
        println!("Enter number to generate the nth fibonacci number");
        let mut nth_iteration_of_fibonacci: String = String::new();
        
            io::stdin()
            .read_line(&mut nth_iteration_of_fibonacci)
            .expect("Failed to read line");
    
        let nth_iteration_of_fibonacci: u32 = match nth_iteration_of_fibonacci.trim().parse::<u32>() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Invalid Number");
                continue;
            },
        };
            break fibonacci_calculator(nth_iteration_of_fibonacci)
        } 
}


fn main() {
    println!("{0}", user_prompt(calculate_fibonacci));
}
