fn main() {
    println!("Hello, world!");


}

fn hexDecoder(hex_string:&str) -> String {
    let mut hex_vector_1: Vec<char> = hex_string_1.chars().collect();
    let mut hex_vector_1: Vec<char> = hex_string_2.chars().collect();
    while hex_vector_1.len() > 0 {
        let mut hex_1 = hex_vector_1.pop().unwrap().to_digit(16).unwrap() as u8;
        let mut hex_2 = hex_vector_2.pop().unwrap().to_digit(16).unwrap() as u8;
        let mut hex_xor = 0;
        for x in 1 ... 5 {
            hex_xor = hex_xor + xorBit(hex_1 % 2, hex_2 % 2);
            hex_1 = (hex_1 - hex_1 % 2)/2;
            hex_2 = (hex_2 - hex_2 % 2)/2;
        }
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
