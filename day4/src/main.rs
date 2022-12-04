use std::fs;
fn main() {
   let file = fs::read_to_string("input.txt").expect("could not read input: ");
   let mut sum = 0;
   let groups = file.split("\n");
   for group in groups {
        let teams = group
            .split(",");
        let mut bounds: Vec<(i32, i32)> = Vec::new();
       for team in teams {
            let sections = team.split("-");
            let res: Vec<_> = sections
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            bounds.push((res[0], res[1]));
        }

        if has_overlap(bounds[0], bounds[1]) {
            println!("{:?} and {:?} overlap", bounds[0], bounds[1]);
            sum += 1;
        }
   }

   println!("{}", sum);
}

fn has_overlap(left: (i32, i32), right: (i32, i32)) -> bool {
    return (right.0 >= left.0 && right.0 <= left.1) ||
            (right.0 <= left.0 && right.1 >= left.1) ||
            (right.0 <= left.0 && right.1 >= left.0)
          
}