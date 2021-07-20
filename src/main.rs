use std::char;
use std::cmp::Ordering;

fn main() {
    let mut num: Vec<char> = Vec::new();
    for count in 0..10 {
        num.push(char::from_digit(count as u32, 10).unwrap());
    }

    let mut alphabet: Vec<char> = Vec::with_capacity(26);
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

    let mut symbol: Vec<char> = Vec::with_capacity(16);
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

    let mut alphabet_morse: Vec<String> = Vec::with_capacity(26);
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

    let mut symbol_morse: Vec<String> = Vec::with_capacity(16);
    // period
    symbol_morse.push(String::from(".-.-.-"));
    // comma
    symbol_morse.push(String::from("--..--"));
    // colon
    symbol_morse.push(String::from("---..."));
    // semicolon
    symbol_morse.push(String::from("-.-.-."));
    // question mark
    symbol_morse.push(String::from("..--.."));
    // apostrophe '
    symbol_morse.push(String::from(".----."));
    // hyphen
    symbol_morse.push(String::from("-....-"));
    // slash
    symbol_morse.push(String::from("-..-."));
    // double dush
    symbol_morse.push(String::from("-...-"));
    // plus sign
    symbol_morse.push(String::from(".-.-."));
    // quotation mark "
    symbol_morse.push(String::from(".-..-."));
    // at sign
    symbol_morse.push(String::from(".--.-."));
    // doller sign
    symbol_morse.push(String::from("...-..-"));
    // underscore
    symbol_morse.push(String::from("..--.-"));
    // ampersand
    symbol_morse.push(String::from(".-..."));
    // exclamation point
    symbol_morse.push(String::from("-.-.--"));

    let search_char: char = '　';
    let char_vec: Option<Vec<char>> = select_char_vec(&search_char, num, alphabet, symbol);
    println!("char_vec is {:?}", char_vec);
    let index: Option<usize> = get_index(&char_vec, search_char);
    let morse_vec: Option<Vec<String>> =
        select_morse_vec(char_vec, num_morse, alphabet_morse, symbol_morse);

    let val: String = get_val(&search_char, index, morse_vec);

    println!("val is {}", val);
}

fn select_char_vec(
    search_char: &char,
    num: Vec<char>,
    alphabet: Vec<char>,
    symbol: Vec<char>,
) -> Option<Vec<char>> {
    match &search_char.is_ascii_digit() {
        true => Some(num),
        false => match &search_char.is_ascii_alphabetic() {
            true => Some(alphabet),
            false => match symbol.contains(&search_char) {
                true => Some(symbol),
                false => None,
            },
        },
    }
}

fn get_index(char_vec: &Option<Vec<char>>, search_char: char) -> Option<usize> {
    match char_vec {
        None => None,
        Some(i) => {
            let charactor = String::from(search_char).to_lowercase();
            let index: Option<usize> = i
                .iter()
                .position(|&x| x == charactor.chars().nth(0).unwrap());
            println!("index is {:?}", index);
            index
        }
    }
}

fn select_morse_vec(
    char_vec: Option<Vec<char>>,
    num_morse: Vec<String>,
    alphabet_morse: Vec<String>,
    symbol_morse: Vec<String>,
) -> Option<Vec<String>> {
    match char_vec {
        None => None,
        Some(i) => {
            let len = i.len();
            match len.cmp(&num_morse.len()) {
                Ordering::Equal => Some(num_morse),
                _ => match len.cmp(&alphabet_morse.len()) {
                    Ordering::Equal => Some(alphabet_morse),
                    _ => match len.cmp(&symbol_morse.len()) {
                        Ordering::Equal => Some(symbol_morse),
                        _ => None,
                    },
                },
            }
        }
    }
}

fn get_val(search_char: &char, index: Option<usize>, morse_vec: Option<Vec<String>>) -> String {
    match search_char.is_whitespace() {
        true => String::from("   "),
        false => match index {
            None => panic!("index is None"),
            Some(i) => match morse_vec {
                None => panic!("morse_vec is None"),
                Some(j) => j[i].to_string(),
            },
        },
    }
}
