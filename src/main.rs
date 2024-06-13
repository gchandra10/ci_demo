mod calculator;

fn main() {
    let num1: f32 = 34.0;
    let num2: f32 = 17.0;
    let op = "+";
    let result: f32;

    match op {
        "+" => result = calculator::add(num1, num2),
        "-" => result = calculator::sub(num1, num2),
        "*" => result = calculator::mul(num1, num2),
        "/" => result = calculator::div(num1, num2),
        _ => panic!("Invalid Operator"),
    }

    // if op == "+" {
    //     result = calculator::add(a, b);
    // } else if op == "-" {
    //     result = calculator::sub(a, b)
    // }
    // else if op == "*" {
    //     result = calculator::mul(a, b)
    // }
    // else if op == "/" {
    //     result = calculator::div(a, b)
    // }

    println!("\n\n Result of {num1} {op} {num2} = {result}\n\n");

}
