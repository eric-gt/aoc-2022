use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Split;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("failed to read file: ");
    total_priority(file.split("\n"));
    trio_priority(file.split("\n"));
}

fn trio_priority(rucksacks: Split<&str>) {
    let mut repeat_maps: HashMap<i32, HashSet<char>> = HashMap::new();
    let mut sum = 0;
    let mut count = 0;
    
    for sack in rucksacks {
        let items = string_to_vec(sack);
        let item_set = HashSet::from_iter(items.iter().cloned());
        repeat_maps.insert(count, item_set);
        if (count + 1) % 3 == 0 {
            println!("prioritizing trio {}", count/3);
            let first = repeat_maps.get(&(count - 2)).unwrap();
            let second = repeat_maps.get(&(count - 1)).unwrap();
            let third = repeat_maps.get(&count).unwrap();

            for item in first {
                if second.contains(&item) && third.contains(&item) {
                    println!("trio {} has item {}", count/3, item);
                    sum += get_priority(*item);
                }
            }
        }
        count += 1;
    }

    println!("{}", sum)
}

fn total_priority(rucksacks: Split<&str>) {
    let mut total_repeats: HashMap<char, i32> = HashMap::new();
    for sack in rucksacks {
        let split= sack.split_at(sack.len()/2);
        let pockets = (string_to_vec(split.0), string_to_vec(split.1));

        let sack_repeats = find_repeats (pockets.0, pockets.1);
        for item in sack_repeats {
            if !total_repeats.contains_key(&item){
                total_repeats.insert(item, 1);
            }
            else {
                let old_count = total_repeats[&item];
                total_repeats.insert(item, old_count + 1);
            }
        }
    }
    let mut sum: i32 = 0;
    let mut add_count = 0;
    for pair in total_repeats.into_iter() {
        sum += get_priority(pair.0) * pair.1;
        add_count += pair.1;
    }
    println!("sum: {}, add_count: {}", sum, add_count);
}

fn find_repeats(left: Vec<char>, right: Vec<char>) -> HashSet<char> {
    let mut repeats: HashSet<char> = HashSet::new();

    for item in left.into_iter(){
        if right.contains(&item) {
            repeats.insert(item);
        }
    } 
    return repeats;
}

fn string_to_vec(arg_string: &str) -> Vec<char> {
    return String::from(arg_string).chars().collect();
}
fn get_priority(item: char) -> i32 {
    let vals = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let index = vals.iter().position(|val| val == &item).unwrap();
    return index as i32+ 1;
}
