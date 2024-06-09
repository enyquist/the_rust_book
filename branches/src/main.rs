fn main() {
    temperature_exercise();
    fibonacci_exercise();
}

fn fibonacci_exercise() {
    // get user input
    let mut input = String::new();
    println!("Enter the number of the fibonacci sequence:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Please enter a number");

    let fibonacci = dynamic_programming_fibonacci(n);

    println!("The {n}th fibonacci number is: {}", fibonacci);

}

fn temperature_exercise() {
    // get user input
    let mut input = String::new();
    println!("Enter the temperature:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temperature: f64 = input.trim().parse().expect("Please enter a number");

    // get user input
    let mut input = String::new();
    println!("Enter the unit of the temperature (fahrenheit (f) or celsius (c)):");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let from = input.trim();

    // get other temperature unit
    let to = match from {
        "f" => "c",
        "c" => "f",
        _ => panic!("Please enter f or c"),
    };

    let converted_temperature = convert_temperature(temperature, from, to);

    println!("The temperature is: {} {}", converted_temperature, to);
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn convert_temperature(temperature: f64, from: &str, to: &str) -> f64 {
    match (from, to) {
        ("f", "c") => fahrenheit_to_celsius(temperature),
        ("c", "f") => celsius_to_fahrenheit(temperature),
        _ => temperature,
    }
}


// Generate the nth fibonacci number using dynamic programming
// Also known as memoization
// Prepare an array to store the fibonacci numbers
fn dynamic_programming_fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib = Vec::with_capacity(n + 1);
    fib.push(0);  // 0th fibonacci number
    fib.push(1);  // 1st fibonacci number

    for i in 2..=n {
        let next_fib = fib[i - 1] + fib[i - 2];
        fib.push(next_fib);
    }

    fib[n]
}
