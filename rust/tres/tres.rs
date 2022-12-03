use std::fs;

fn main() {
    let filepath = "./input.txt";

    // let mut players: [i32];

    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut total_priority: i64 = 0;
    // let mut max3_cals = [1, 1, 1]; // top 3 elves total calories
    for line in lines {
        let line_len = line.len();
        let first_half = &line[..line_len / 2];
        let second_half = &line[line_len / 2..line_len];
        println!("{}-{}", first_half, second_half);
        let mut priority: i64 = 0;
        for (i, &first_item) in first_half.as_bytes().iter().enumerate() {
            for (j, &second_item) in second_half.as_bytes().iter().enumerate() {
                if first_item == second_item {
                    let duplicate = &line[i..i + 1];
                    priority = if first_item >= 97 {
                        // lowercase
                        (first_item - 96).into()
                    } else {
                        // uppercase
                        (first_item - 38).into()
                    };
                    println!("duplicate {}, {}, {}", duplicate, first_item, priority);
                    // A = 65, B = 66,  Z = 90
                    // a = 97, z = 122
                    break;
                }
            }
        }
        total_priority += priority;
        println!("total piority {}", total_priority);
    }

    println!("total priority {}", total_priority);
}
