use rand::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

mod caeser;

fn main() {
    let mut rng = thread_rng();
    let path = Path::new("library.txt");

    let mut file = match File::open(path) {
        Err(err) => panic!("err: {}", err),
        Ok(file) => file
    };

    let mut lines = String::new();
    match file.read_to_string(&mut lines) {
        Err(err) => panic!("err: {}", err),
        Ok(_) => (),
    }

    let split_lines: Vec<&str> = lines.trim().split("\n").collect();
    let rand_idx: usize = rng.gen_range(0..split_lines.len());
    let plain_text = split_lines[rand_idx];
    let cipher_text = caeser::caesar_cipher(plain_text.to_string(), rand_idx+3);
    let mut working_solution = cipher_text;
    let width = working_solution.len();
    let barrier = format!("{:-<width$}", "");

    // assign each letter a number
    let mut lookup_table: HashMap<char, u8> = HashMap::new();
    let mut counter: u8 = 0;
    for letter in plain_text.to_lowercase().chars() {
        match lookup_table.get(&letter) {
            Some(_e) => (),
            None => if letter != ' ' {
                counter += 1;
                lookup_table.insert(letter, counter);
            }
        }
    }

    // make string that user will see to show each letter's number
    let mut first_row: Vec<String> = vec!();
    let mut second_row: Vec<String> = vec!();
    //let mut second_row: Vec<char> = vec!();
    for letter in plain_text.to_lowercase().chars() {
        if letter != ' ' {
            if lookup_table.get(&letter).expect("letter in plain_text not found in lookup_table (really shouldnt be possible)").to_string().len() == 1 {
                first_row.push(format!("{}", *lookup_table.get(&letter).expect("letter in plain_text not found in lookup_table")));
                second_row.push(format!(" "));
            } else if lookup_table.get(&letter).expect("letter in plain_text not found in lookup_table (really shouldnt be possible)").to_string().len() == 2 {
                //this is absolutely terrible code and i should probably fix this later. i wont though
                let num = lookup_table.get(&letter).expect("this is a bug that really shouldnt happen.");
                let split_num = &num.to_string().chars().collect::<Vec<char>>();
                let first_num = split_num.get(0).unwrap();
                let second_num = split_num.get(1).unwrap();
                first_row.push(format!("{}", first_num));
                second_row.push(format!("{}", second_num));
            }
        } else {
            first_row.push(String::from(" "));
            second_row.push(String::from(" "));
        }
    }
    let first_row_string = first_row.into_iter().collect::<String>();
    let second_row_string = second_row.into_iter().collect::<String>();
    print!("{}\n", working_solution);
    print!("{}\n", barrier);
    print!("{}\n", first_row_string);
    print!("{}\n", second_row_string);

    /*while working_solution != plain_text {
        //print!("{}\n", working_solution);
        //print!("{}\n", barrier);

        for (key, value) in lookup_table {
            print!("{}, {}\n", key, value);
        }
        /*
            TODO: make a thing that puts numbers under the letters, with the first encountered letter being 1, second being 2,
            ex: in "the quick brown fox jumps over the lazy dog", t=1, h=2, e=3, etc. also make it so that numbers with 2 digits
            are printed vertically. final print out would look like ex:
            asdfj asdfasjdfkla fasdhjflom
            -----------------------------
            12345 123412534671 4123854791
                                        0
        */
    }*/
}