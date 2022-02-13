use std::io;
fn main() {
    let mut string = String::new();
    println!("Enter a String for Palindrome check: ");
    io::stdin().read_line(&mut string);
}
//Use function Palindrome in Rust
use regex::Regex;
fn check_palindrome(input: String) {
    let msg_string: String = string.trim().to_lowecase();
    let key: &str = msg_string.as_str();
    let mut Palindeome: bool = true;
    
// for

    for (index, letter) in key.chars().enumerate()  {
        if index == key.len() /2 { 
            break;
        }
        
        if letter !=key.chars().rev().nth(index).unwrap() {
                Palindeome = false;
            break;
        }
    }
    if Palindeome {
        println!("\"{}\" is a palindrome:", string.trim());
    }  else {
        println!("\"{}\" is not a palindrome:", string.trim());
    }
}

