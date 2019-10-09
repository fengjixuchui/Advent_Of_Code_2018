mod utility;
use utility::utility::*;
use utility::stopwatch::stopwatch::*;
use std::collections::HashMap;

fn part_2() {
    let input = load_file("resources/day_2_input.txt");
    let stopwatch = Stopwatch::start_new();
    let ids = split_by_new_line_as_char_vector(input);

    let mut matching_string = String::from("");

    'first_loop: for index1 in 0..ids.len() {
        let id = &ids[index1];
        'second_loop: for index2 in index1+1..ids.len() {
            let id2 = &ids[index2];
            let mut current_matching_string = String::from("");
            let mut difference = 0;
            for i in 0..id.len() {
                let char1 = id[i];
                let char2 = id2[i];
                if char1 == char2 {
                    current_matching_string.push(char1);
                } else {
                    difference += 1;
                }
                if difference > 1 {
                    continue 'second_loop;
                }
            }
            if difference == 1 {
                matching_string = current_matching_string;
                break 'first_loop;
            }
        }
    }
    stopwatch.print_elapsed();
    println!("Result: {}", matching_string);
}

fn part_1() {
    let input = load_file("resources/day_2_input.txt");
    let stopwatch = Stopwatch::start_new();
    let ids = split_by_new_line(input);

    let mut ids_with_same_letter_twice = 0;
    let mut ids_with_same_letter_thrice = 0;

    for id in ids {
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        for character in id.chars() {
            match char_counts.get(&character) {
                Some(amount) => {
                    let new_amount = amount + 1;
                    char_counts.insert(character, new_amount);
                },
                _ => {
                    char_counts.insert(character, 1);
                }
            };
        }
        let mut contains_same_letter_twice = false;
        let mut contains_same_letter_thrice = false;
        for (_, amount) in char_counts {
            if amount == 2 {
                contains_same_letter_twice = true;
            } else if amount == 3 {
                contains_same_letter_thrice = true;
            }
        }
        if contains_same_letter_twice {
            ids_with_same_letter_twice += 1;
        }
        if contains_same_letter_thrice {
            ids_with_same_letter_thrice += 1;
        }
    }
    stopwatch.print_elapsed();
    println!("Result: {}", (ids_with_same_letter_twice * ids_with_same_letter_thrice));
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
