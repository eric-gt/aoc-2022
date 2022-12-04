use std::fs;
fn main() {
    let file = fs::read_to_string("input.txt").expect("failed to read file: ");
    let rounds = file.split("\n");
    let mut sum = 0;
    for round in rounds {
        let split_string: Vec<&str> = round
            .split(" ").collect();
        let moves = (split_string[0], split_string[1]);
        let score = score_match(moves);
        sum += score;
    }
    println!("{}", sum)
}

fn score_match((attack, result ): (&str, &str)) -> i32 {
    match (attack, result) {
        ("A", "X") => return 3,
        ("A", "Y") => return 4,
        ("A", "Z") => return 8,
        ("B", "X") => return 1,
        ("B", "Y") => return 5,
        ("B", "Z") => return 9,
        ("C", "X") => return 2,
        ("C", "Y") => return 6,
        ("C", "Z") => return 7,
        _ => return 0 
    }
}
