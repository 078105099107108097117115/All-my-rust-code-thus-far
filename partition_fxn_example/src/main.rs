use std::collections::HashSet;
use std::io::{Write, self};
fn main() {
    let x = 4;
    let y = 10;
    println!("x & (x - 1) == 0 -> {}",(x & (x-1) == 0));
    println!("y & (y - 1) == 0 -> {}",(y & (y-1) == 0));
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure) : (HashSet<i32>, HashSet<i32>) = 
                                   squares.iter().partition(|&n| n & (n-1) ==0);
    print!("The squares array = ");
    print!("{:?}", squares);
    io::stdout().flush().unwrap();
    println!("");
    print!("powers_of_two == {:?}",powers_of_two);
    io::stdout().flush().unwrap();
    println!("");
    print!("impure/ non-powers_of_two == {:?}",impure);
    io::stdout().flush().unwrap();
    println!("");
    let a_str = String::from("Mori Morimoto, Uesugi Kenshin, Takeda Shingen, Oda Nobunaga");
    let (upper, lower) : (String, String) = a_str.chars().partition(|&c| c.is_uppercase());
    print!("upper == {};\nlower == {}",upper, lower);
    io::stdout().flush().unwrap();
    println!("");
}
