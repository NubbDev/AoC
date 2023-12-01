use std::vec;
use std::time;

fn main() {
    let start = time::Instant::now();

    let _file = std::fs::read_to_string("./input.txt").expect("File not found");
    let num = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let num_text = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let first = part1(_file.clone(), &num);
    let second = part2(_file.clone(), &num, &num_text);

    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Time: {}ms", start.elapsed().as_millis());
}

fn analyze<'a>(text: &Vec<&'a str>, num_vec: &Vec<&str>) -> &'a str{
    let mut num = "";
    for char in text {
        if num_vec.contains(&char) {
            num = char;
            break;
        }
    }
    num
}

fn part1(file: String, num: &Vec<&str>) -> i32 {
    let mut sum = 0;
    let lines = file.lines();
    for line in lines {
        let mut text: Vec<&str> = line.split("").collect();

        let first_number = analyze(&text, num);
        text.reverse();
        let last_number = analyze(&text, num);

        sum += (String::from(first_number) + last_number).parse::<i32>().unwrap();
    
    }
    sum
}

fn part2(file: String, num: &Vec<&str>, num_text: &Vec<&str>) -> i32 {
    let mut sum = 0;
    
    let lines = file.lines();
    for line in lines {
        let mut temp_text: String = line.to_string().to_lowercase();

        for num_replace in num_text {
            let num_index = num_text.iter().position(|&r| r == num_replace.to_owned()).unwrap();
            let last_letter: Vec<&str> = num_replace.to_owned().split("").collect();

            let replacement = String::from(last_letter[1]) + num[num_index] + last_letter[last_letter.len() - 2];

            temp_text = temp_text.replace(num_replace.to_owned(), &replacement);
        }

        let mut text: Vec<&str> = temp_text.split("").collect();

        let first_number = analyze(&text, num);
        text.reverse();
        let last_number = analyze(&text, num);

        sum += (String::from(first_number) + last_number).parse::<i32>().unwrap();
    }

    sum
}