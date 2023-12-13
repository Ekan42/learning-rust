fn times_two(number: i32) -> i32 {
    number * 2
}
fn main() {
    let final_number = {
    let y = 10;
    let x = 9; 
    let x = times_two(x);
    let x = x + y;
    x
    };
    println!("The number is now: {}", final_number)
}
