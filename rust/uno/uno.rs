use std::fs;

    
fn main () {
    let filepath = "./input.txt";

    // let mut players: [i32];
    
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut max_cals = 0;
    let mut cal_counter = 0;
    for line in lines {
        if line == "" {
            //println!("newline ");
            if cal_counter > max_cals {
                max_cals = cal_counter
            }
            cal_counter = 0
        } else {
            //println!("got {}", line);
            let cal_num = line.parse::<i32>().unwrap();
            cal_counter += cal_num;
        }
    }
    
    println!("max cals {}", max_cals);
    //print!("{}", l)
}
