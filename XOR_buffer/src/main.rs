use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
println!("{}", hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"));
assert_eq!(hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),"746865206b696420646f6e277420706c6179");
//println!("{}", hex_to_ascii("68656c6c6f"));
hex_to_ascii(String::from("68656c6c6f"));
let digrams = xor_decrypter("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
println!("{}", digrams);
}

fn xor_decrypter(str_decrypt: &str) -> String{
    let digrams = digram_counter(str_decrypt);
    let largest = get_largest(digrams);
    //digrams.remove(&largest);
    let xor_key = hex_decoder(&largest, "65");
    let mut test_hex = String::from("");
    while test_hex.len() < str_decrypt.len() {
        test_hex = test_hex.clone() + &xor_key;
    }
    let result = hex_decoder(&test_hex, &str_decrypt);
    return result;
}

fn get_largest(hex_count: HashMap<String, i32>) -> String {
    let mut largest_key =  String::from("");
    let mut largest_value = &i32::from(0);
    for (key, val) in hex_count.iter() {
        if val > largest_value{
            largest_key = key.clone();
            largest_value = val;
        }
    }
    return largest_key;
}
fn digram_counter(hex_string:&str) -> HashMap<String, i32> {
    let mut hex_vector: Vec<char>= hex_string.chars().collect();
    let mut hex_digram = HashMap::new();
    while hex_vector.len() > 0{
        let hex_1 = hex_vector.pop().unwrap() as char;
        let hex_2 = hex_vector.pop().unwrap() as char;
        let hex_double = hex_1.to_string().clone() + &hex_2.to_string();
        if  hex_digram.contains_key(&hex_double){
            *hex_digram.entry(hex_double).or_insert(1) +=1;
        }
        else {
            hex_digram.insert(hex_double, 1);
        }
    }
    return hex_digram.clone();
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
            hex_xor = hex_xor + (2u8).pow(x)*xor_bit(&(hex_1 % 2), &(hex_2 % 2));
            hex_1 = (hex_1 - hex_1 % 2)/2;
            hex_2 = (hex_2 - hex_2 % 2)/2;
        }
        hex_vector_3.push_front(hex_converter[hex_xor as usize]);
     }
	return hex_vector_3.iter().collect();
}


fn xor_bit(hex_1: &u8, hex_2: &u8) -> u8{
    if hex_1.clone() == 1u8 && hex_2.clone() == 1u8 {
        return 0u8;
    }
    else if hex_1.clone() == 1u8 || hex_2.clone() == 1u8 {
        return 1u8;
    }
    else {
        return 0u8;
    }
}

fn unwrap_vector(hex: Option<char>) -> u8 {
	let x: u8 = hex.unwrap().to_digit(16).unwrap() as u8;
	return x;
}
//hex: Vec<char>


fn hex_to_ascii(hex_encoded_str: String) -> String{
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
	final_string.push(alphabet[decimal_encoded[x]]); //get error handling in this
	}
	println!("{}", final_string);
	return final_string
}
