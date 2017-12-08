use std::collections::VecDeque;

fn main() {
println!("{}", hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"));
assert_eq!(hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),"746865206b696420646f6e277420706c6179")
}

fn hex_decoder(hex_string_1:&str, hex_string_2:&str) -> String {
    let mut hex_vector_1: Vec<char> = hex_string_1.chars().collect();
    let mut hex_vector_2: Vec<char> = hex_string_2.chars().collect();
    let mut hex_vector_3 = VecDeque::new();
    let hex_converter: Vec<char> = "0123456789abcdef".chars().collect();
    while hex_vector_1.len() > 0 {
        let mut hex_1 = unwrap_vector(hex_vector_1.pop());
        let mut hex_2 = unwrap_vector(hex_vector_2.pop());
        let mut hex_xor = 0;
        for x in 0..4 {
            hex_xor = hex_xor + 2u8.pow(x)*xor_bit(hex_1 % 2, hex_2 % 2);
            hex_1 = (hex_1 - hex_1 % 2)/2;
            hex_2 = (hex_2 - hex_2 % 2)/2;
        }
        hex_vector_3.push_front(hex_converter[hex_xor as usize]);
    }
	return hex_vector_3.iter().collect();
}

fn xor_bit(hex_1: u8, hex_2: u8) -> u8{
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

