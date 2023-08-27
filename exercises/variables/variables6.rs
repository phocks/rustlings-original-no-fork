// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut number: u8 = 255;
    println!("Number {}", number);
    number = number.wrapping_add(1);
    println!("Number {}", number);
}
