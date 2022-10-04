use std::env;
use std::process;
use std::char;
fn main(){
    let args: Vec<String> = env::args().collect();
    let input_string: String;
    if args.len() == 2{
        input_string = args[1].parse::<String>().unwrap();       
    }
    else{
        println!("No argument given.");
        process::exit(0x0001);
    }
    let char_vec: Vec<char> = input_string.chars().collect();
    let mut str_end: usize = char_vec.len() - 1;
    let mut str_start: usize = 0;
    let mut not_a_palindrome: bool = false;
    while str_start != str_end && str_start < str_end{
        if char_vec[str_start] == char_vec[str_end]{
            str_end = str_end - 1;
            str_start = str_start + 1;
            continue;
        }
        else{
            not_a_palindrome = true;
            break;
        }
    }
    if not_a_palindrome == true{
        print!("\"{}\" is not a palindrome...", &input_string);
    }
    else{
        print!("\"{}\" is a palindrome!", &input_string);
    }
    
}
