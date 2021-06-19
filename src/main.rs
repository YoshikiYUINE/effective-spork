use std::char;
fn main() {
    let mut num: Vec<char> = Vec::new();
    for count in 0..10 {
        num.push(char::from_digit(count as u32, 10).unwrap());
    }

    let mut alphabet: Vec<char> = Vec::new();
    alphabet.push('a');
    alphabet.push('b');
    alphabet.push('c');
    alphabet.push('d');
    alphabet.push('e');
    alphabet.push('f');
    alphabet.push('g');
    alphabet.push('h');
    alphabet.push('i');
    alphabet.push('j');
    alphabet.push('k');
    alphabet.push('l');
    alphabet.push('m');
    alphabet.push('n');
    alphabet.push('o');
    alphabet.push('p');
    alphabet.push('q');
    alphabet.push('r');
    alphabet.push('s');
    alphabet.push('t');
    alphabet.push('u');
    alphabet.push('v');
    alphabet.push('w');
    alphabet.push('x');
    alphabet.push('y');
    alphabet.push('z');

    let mut symbol: Vec<char> = Vec::new();
    // period
    symbol.push('.');
    // comma
    symbol.push(',');
    // colon
    symbol.push(':');
    // semicolon
    symbol.push(';');
    // question mark
    symbol.push('?');
    // apostrophe '
    symbol.push('\u{0027}');
    // hyphen
    symbol.push('-');
    // slash
    symbol.push('/');
    // double dush
    symbol.push('=');
    // plus sign
    symbol.push('+');
    // quotation mark "
    symbol.push('\u{0022}');
    // at sign
    symbol.push('@');
    // doller sign
    symbol.push('$');
    // underscore
    symbol.push('_');
    // ampersand
    symbol.push('&');
    // exclamation point
    symbol.push('!');

    let mut num_morse: Vec<String> = Vec::new();
    num_morse.push(String::from("-----"));
    num_morse.push(String::from(".----"));
    num_morse.push(String::from("..---"));
    num_morse.push(String::from("...--"));
    num_morse.push(String::from("....-"));
    num_morse.push(String::from("....."));
    num_morse.push(String::from("-...."));
    num_morse.push(String::from("--..."));
    num_morse.push(String::from("---.."));
    num_morse.push(String::from("----."));

    let mut alphabet_morse: Vec<String> = Vec::new();
    alphabet_morse.push(String::from(".-"));
    alphabet_morse.push(String::from("-..."));
    alphabet_morse.push(String::from("-.-."));
    alphabet_morse.push(String::from("-.."));
    alphabet_morse.push(String::from("."));
    alphabet_morse.push(String::from("..-."));
    alphabet_morse.push(String::from("--."));
    alphabet_morse.push(String::from("...."));
    alphabet_morse.push(String::from(".."));
    alphabet_morse.push(String::from(".---"));
    alphabet_morse.push(String::from("-.-"));
    alphabet_morse.push(String::from(".-.."));
    alphabet_morse.push(String::from("--"));
    alphabet_morse.push(String::from("-."));
    alphabet_morse.push(String::from("---"));
    alphabet_morse.push(String::from(".--."));
    alphabet_morse.push(String::from("--.-"));
    alphabet_morse.push(String::from(".-."));
    alphabet_morse.push(String::from("..."));
    alphabet_morse.push(String::from("-"));
    alphabet_morse.push(String::from("..-"));
    alphabet_morse.push(String::from("...-"));
    alphabet_morse.push(String::from(".--"));
    alphabet_morse.push(String::from("-..-"));
    alphabet_morse.push(String::from("-.--"));
    alphabet_morse.push(String::from("--.."));

    let mut symbol_morse: Vec<String> = Vec::new();
    // period
    symbol_morse.push(String::from(".-.-.-"));
    // comma
    symbol_morse.push(String::from("--..--"));
    // colon
    symbol_morse.push(String::from("---..."));
    // semicolon
    // question mark
    // apostrophe '
    // hyphen
    // slash
    // double dush
    // plus sign
    // quotation mark "
    // at sign
    // doller sign
    // underscore
    // ampersand
    // exclamation point

    let search_char: char = '8';
    let num_index: i32 = get_index(&num, search_char);

    let index: usize = check_index(num_index);

    // let val: i32 = num[index];
    let val: String = num_morse[index].to_string();

    println!("num_index is {}", num_index);
    println!("val is {}", val);
    println!("len is {}", num.len());
}

fn get_index(num: &Vec<char>, search_char: char) -> i32 {
    let index: Option<usize> = num.iter().position(|&x| x == search_char);
    println!("index is {:?}", index);
    match index {
        None => -1,
        Some(i) => i as i32,
    }
}

fn check_index(index_value: i32) -> usize {
    match index_value {
        -1 => panic!("index error"),
        _ => index_value as usize,
    }
}
