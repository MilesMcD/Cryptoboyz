use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    println!("{}", hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"));
    assert_eq!(hex_decoder("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),"746865206b696420646f6e277420706c6179");
    let mut digrams = xor_decrypter("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("{}", digrams);
}

fn xor_decrypter(str_decrypt: &str) -> String{
    let mut digrams = digramCounter(str_decrypt);
    let largest = getLargest(digrams);
    digrams.remove(largest);
    let xor_key = hex_decoder(largest, "65");
    let mut test_hex = "";
    while test_hex.len() < str_decrypt.len() {
        test_hex.push_str(xor_key);
    }
    let result = hex_decoder(test_hex, str_decrypt);
    return result;
}

fn getLargest(hex_count: HashMap<String, i32>) -> String {
    let mut largest_key =  "";
    let mut largest_value = 0;
    for (key, val) in hex_count.iter() {
        if val > largest_value{
            largest_key = key;
            largest_value = val;
        }
    }
    return largest_key;

}
fn digramCounter(hex_string:&str) -> HashMap<String, i32> {
    let mut hex_vector = hex_string.chars().collect();
    let mut hex_digram = HashMap::new();
    while hex_vector.len() > 0{
        let hex_1 = hex_vector.pop().unwrap() as char;
        let hex_2 = hex_vector.pop().unwrap() as char;
        let hexDouble = hex_1.to_string().push_str(hex_2.to_string());
        if  hex_digram.contains_key(hexDouble){
            *hex_digram.entry(hexDouble).or_insert(1) +=1;
        }
        else {
            hex_digram.insert(hexDouble, 1);
        }
    }
    return hex_digram;
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
