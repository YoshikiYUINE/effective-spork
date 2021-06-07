fn main() {
    let mut num: Vec<i32> = Vec::new();
    for count in 0..9 {
        num.push(count);
    }
    println!("num is {:?}", num);

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

    let search_num_char: i32 = 8;
    let num_index: i32 = get_num_index(&num, search_num_char);

    let index: usize = check_index(num_index);

    // let val: i32 = num[index];
    let val: String = num_morse[index].to_string();

    println!("num_index is {}", num_index);
    println!("val is {}", val);
    println!("len is {}", num.len());
}

fn get_num_index(num: &Vec<i32>, search_num_char: i32) -> i32 {
    let index: Option<usize> = num.iter().position(|&x| x == search_num_char);
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
