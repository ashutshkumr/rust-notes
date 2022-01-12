
use std::io::{self, Write};


pub fn sort(numbers: &mut Vec<i32>) {
    let mut tmp: i32;

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] > numbers[j] {
                tmp = numbers[i];
                numbers[i] = numbers[j];
                numbers[j] = tmp;
            }
        }
    }
}

pub fn get_input_and_sort() {
    print!("Enter integers separated by space: ");
    io::stdout().flush().expect("could not flush stdout");

    let mut buf =  String::new();
    io::stdin().read_line(&mut buf).expect("could not read integers");

    let mut numbers:Vec<i32> = Vec::new();
    for result in buf.split_ascii_whitespace().map(|x| x.parse::<i32>()) {
        match result {
            Ok(val) => numbers.push(val),
            Err(e) => panic!("could not parse integers: {}", e)
        }
    }

    sort(&mut numbers);
    print!("Sorted integers: {:?}", numbers);
}