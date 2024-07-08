// Define the print function with parameter types
fn print(x: i32, y: u64) {
    println!("signed integer  :{}", x);
    println!("unsigned integer  :{}", y);
}

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    print(x, y); // Correct the function call with a semicolon
}
