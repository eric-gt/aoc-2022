use std::fs;
fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read file: ");
    let blocks= file.split("\n\n");
    let input: Vec<&str> = blocks.collect();

    let mut elves:Vec<i32> = Vec::new();
    for i in 0..input.len() {
        let food: Vec<&str> = input[i].split("\n").collect();
        let cal_count = strings_to_sum(food);
        elves.push(cal_count);
    }
    elves.sort();
    elves.reverse();
    let max = elves[0] + elves[1] + elves[2];
    println!("{}", max);
}

fn strings_to_sum(strings: Vec<&str>) -> i32{
    let mut sum = 0;
    for s in strings{
        let n = s.parse::<i32>().expect("unable to parse: ");
        sum += n;
    }
    return sum;
}
