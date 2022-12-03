use std::fs;

fn main() {
    let filepath = "./input.txt";

    // let mut players: [i32];

    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut elf_calories: Vec<i64> = vec![];
    // let mut max3_cals = [1, 1, 1]; // top 3 elves total calories
    let mut cal_counter = 0; // stores calories per elf
    for line in lines {
        if line == "" {
            // finished counting an elf, compare it to max
            elf_calories.push(cal_counter);
            cal_counter = 0;
        } else {
            //println!("got {}", line);
            let cal_num = line.parse::<i64>().unwrap();
            cal_counter += cal_num;
        }
    }
    elf_calories.sort_by(|a, b| b.cmp(a));

    println!("max elf {}", elf_calories[0]);
    println!("{:?}", elf_calories[0] + elf_calories[1] + elf_calories[2]);

}
