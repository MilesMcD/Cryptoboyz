use std::collections::VecDeque;
fn main() {
println!("{}", to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
//49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
}

fn to_base64(hex_string: &str) -> String {
    //convert string into vector
    let mut hex_vector: Vec<_> = hex_string.chars().collect();
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();
    let mut base64 = VecDeque::new();
	while hex_vector.len() > 2 {
        let first_hex = unwrap_vector(hex_vector.pop());//gets first hex character
        let mut second_hex = unwrap_vector(hex_vector.pop());//gets second hex character
        let third_hex = unwrap_vector(hex_vector.pop());//gets third hex character
        let second_hex_relevant = second_hex % 4;//mods the second hex character by four to get data in first to binary digits
        let value_1 = second_hex_relevant * 16 + first_hex;//multiplies the value gotten from the second hex above by 16 to account for place and adds value from second hex
        second_hex = second_hex - second_hex_relevant;//removes the value used in calculating the first base 64 number from second hex
        second_hex = second_hex/4;//divides this by four to account for place ie so 1100 = 11
        let value_2 = third_hex*4 + second_hex;//multiplies the third hex by 4 to account for places and addds leftover value from the second hex

        base64.push_front(alphabet[value_1 as usize]);//converts the first integer value into base 64 and adds it to the base 64 vector
        base64.push_front(alphabet[value_2 as usize]);//converts the second integer value into base 64 and adds it to the base 64 vector
    }
    if hex_vector.len() > 0{//handles if the hex value is not divisible by 3 and there are leftover hex digits
		let first_hex = unwrap_vector(hex_vector.pop()); //grabs the first hex number and converts to integer value
		if hex_vector.len() > 0{//if there is a second hex value left do this
			let mut second_hex = unwrap_vector(hex_vector.pop());//gets the second hex value left
			let second_hex_relevant = second_hex % 4;//takes that hex value mod 4 to get the first to binary digits in the second hex
			let value_1 = second_hex_relevant * 16 + first_hex;//converts the number gotten to account for place and adds the value of the first hex to get the first base64 number
			second_hex = second_hex - second_hex_relevant;//removes the value used in the first base 64 number from second_hex
			base64.push_front(alphabet[value_1 as usize]);//adds the first base 64 number to the vector base64
            if second_hex>0 {
                base64.push_front(alphabet[second_hex/4 as usize]);//creates a second base64 number from the leftover piece from second hex and converts it to account for place
            }
        }
        else {
            base64.push_front(alphabet[first_hex as usize]);//adds the first hex number to the vector if there is no second hex
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
