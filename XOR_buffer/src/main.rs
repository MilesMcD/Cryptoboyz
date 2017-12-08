use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
println!("{}", hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"));
assert_eq!(hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),"746865206b696420646f6e277420706c6179");
//println!("{}", hex_to_ascii("68656c6c6f"));
hex_to_ascii(String::from("68656c6c6f"));
    let mut digrams = xor_decrypter("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
}

fn hex_decoder(hex_string_1:&str, hex_string_2:&str) -> String {
	if hex_string_1.len() != hex_string_2.len()
	{
		panic!("strings are of differing lengths");
	}

    let mut hex_vector_1: Vec<char> = hex_string_1.chars().collect();
    let mut hex_vector_2: Vec<char> = hex_string_2.chars().collect();
    let mut hex_vector_3 = VecDeque::new();
    let hex_converter: Vec<char> = "0123456789abcdef".chars().collect();
    while hex_vector_1.len() > 0 {
        let mut hex_1 = hex_vector_1.pop().unwrap().to_digit(16).unwrap() as u8;
        let mut hex_2 = hex_vector_2.pop().unwrap().to_digit(16).unwrap() as u8;
        let mut hex_xor = 0;
        for x in 0 .. 4 {
            hex_xor = hex_xor + 2.pow(x)*xorBit(hex_1 % 2, hex_2 % 2);
            hex_1 = (hex_1 - hex_1 % 2)/2;
            hex_2 = (hex_2 - hex_2 % 2)/2;
        }
        hex_vector_3.push_front(hex_converter[hex_xor as usize]);
     }
	return hex_vector_3.iter().collect();
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
//hex: Vec<char>


fn hex_to_ascii(hex_encoded_str: String){
	let alphabet: Vec<char> = 
	" !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz".chars().collect();
	let mut final_string: String = String::from("");
	
	let hex: Vec<_> = hex_encoded_str.chars().collect();
	
    let decimal_encoded: Vec<usize> = hex.chunks(2).map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap() as usize;
        let second_byte = chunk[1].to_digit(16).unwrap() as usize;

        ((first_byte * 16) + second_byte - 32)//Alphabet starts at position 32 of ASCII chart
    }).collect();
	
	for x in 0..decimal_encoded.len() {
	final_string.push(alphabet[decimal_encoded[x]]);
	}
	println!("{}", final_string);
	
}
