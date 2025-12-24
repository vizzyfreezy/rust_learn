fn main (){
  math_ops();
  tuples();
}
fn math_ops() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1 becausd theh are integers so it wont produce a float but an int type (int/int==int)
    println!("Quotient is {quotient}");
    println!("Quotient is {quotient}");
    

    // remainder
    let remainder = 43 % 5;
}
fn tuples() {
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;//access item in tuple at index 0
    x.1 += 5;
}