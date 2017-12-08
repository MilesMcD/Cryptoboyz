use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");


}

fn hexDecoder(hex_string_1:&str, hex_string_2:&str) -> String {
    let mut hex_vector_1: Vec<char> = hex_string_1.chars().collect();
    let mut hex_vector_2: Vec<char> = hex_string_2.chars().collect();
    let mut hex_vector_3 = VecDeque::new();
    let hex_converter: Vec<char> = "0123456789abcdef".chars().collect();
    while hex_vector_1.len() > 0 {
        let mut hex_1 = unwrap_vector(hex_vector_1.pop());
        let mut hex_2 = unwrap_vector(hex_vector_2.pop());
        let mut hex_xor = 0;
        for x in 0 ... 4 {
            hex_xor = hex_xor + 2.pow(x)*xorBit(hex_1 % 2, hex_2 % 2);
            hex_1 = (hex_1 - hex_1 % 2)/2;
            hex_2 = (hex_2 - hex_2 % 2)/2;
        }
        base64.push_front(hex_converter[hex_xor as usize]);
    }
}

fn xorBit(hex_1: &u8, hex_2: &u8) -> u8{
    if hex_1 == 1 && hex_2 == 1 {
        return 0;
    }
    else if hex_1 == 1 || hex_2 == 1 {
        return 1;
    }
    else {
        return 0;
    }
}
fn unwrap_vector(hex: Option<char>) -> u8 {
	let x: u8 = hex.unwrap().to_digit(16).unwrap() as u8;
	return x;
}

