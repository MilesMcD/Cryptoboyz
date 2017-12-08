use std::collections::VecDeque;
fn main() {
println!("{}", to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
//49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
}

fn to_base64(hex_string: &str) -> String {
        let mut hex_vector: Vec<_> = hex_string.chars().collect();
        let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();
        let mut base64 = VecDeque::new();
		while hex_vector.len() > 2 {
            let first_hex = unwrap_vector(hex_vector.pop());
            let mut second_hex = unwrap_vector(hex_vector.pop());
            let third_hex = unwrap_vector(hex_vector.pop());
            let second_hex_relevant = second_hex % 4;
            let value_1 = second_hex_relevant * 16 + first_hex;
            second_hex = second_hex - second_hex_relevant;
            second_hex = second_hex/4;
            let value_2 = third_hex*4 + second_hex;

            base64.push_front(alphabet[value_1 as usize]);
            base64.push_front(alphabet[value_2 as usize]);
        }
        if hex_vector.len() > 0{
				let first_hex	 = unwrap_vector(hex_vector.pop());
				if hex_vector.len() > 0{
					let mut second_hex = unwrap_vector(hex_vector.pop());
					let second_hex_relevant = second_hex % 4;
					let value_1 = second_hex_relevant * 16 + first_hex;
					second_hex = second_hex - second_hex_relevant;
					base64.push_front(alphabet[value_1 as usize]);
                if second_hex>0 {
                    base64.push_front(alphabet[second_hex as usize]);
                }
            }
        }
        return base64.iter().collect();
    }
	
fn make_char(number:u8) -> char {
		if number < 10 {
			let x: u8 = number + 48;
			return x as char;
		}
		let hex_strings: Vec<_> = "abcdef".chars().collect();
		let x: char = hex_strings[(number - 10) as usize];
		return x;
}

fn unwrap_vector(hex: Option<char>) -> u8 {
		let x: u8 = hex.unwrap().to_digit(16).unwrap() as u8;
		return x;
}
