//! charactor to morse code.

use std::char;
use std::collections::HashMap;

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

    // create hashmap from vector of num
    let numbers: HashMap<_, _> = num.iter().zip(num_morse.iter()).collect();
    // create hashmap from vector of alphabet
    let alphabets: HashMap<_, _> = alphabet.iter().zip(alphabet_morse.iter()).collect();
    // create hashmap from vector of symbol
    let symbols: HashMap<_, _> = symbol.iter().zip(symbol_morse.iter()).collect();

    let search_word: String = String::from("hello");
    let mut morse_word_vec: Vec<String> = Vec::new();
    for c in search_word.chars() {
        let search_char = c;
        let morse_val: String = get_morse(search_char, &numbers, &alphabets, &symbols);
        morse_word_vec.push(String::from(&morse_val));
        println!("{} of morse code is {}", search_char, morse_val);
    }
    println!("morse word is {:?}",morse_word_vec);
}

/// return morse code
/// from search_char to morse code
/// when search charactor can not change to morse code then error
fn get_morse(
    search_char: char,
    numbers: &HashMap<&char, &String>,
    alphabets: &HashMap<&char, &String>,
    symbols: &HashMap<&char, &String>,
) -> String {
    match search_char.is_whitespace() {
        true => String::from("   "),
        false => {
            let morse = numbers.get(&search_char);
            match morse {
                Some(v) => v.to_string(),
                None => {
                    let morse = alphabets.get(&search_char.to_lowercase().nth(0).unwrap());
                    match morse {
                        Some(v) => v.to_string(),
                        None => {
                            let morse = symbols.get(&search_char);
                            match morse {
                                Some(v) => v.to_string(),
                                None => panic!("get_morse is failed"),
                            }
                        }
                    }
                }
            }
        }
    }
}
