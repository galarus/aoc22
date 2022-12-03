use std::fs;

fn main() {
    let filepath = "./input.txt";

    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let lines = contents.lines();
    for line in lines {
        let mut plays = line.split(" ");
        let enemy_play = plays.next().unwrap();
        let my_play = plays.next().unwrap();
        println!("enemy play {}, my play {}", enemy_play, my_play);
       // for play in plays {
       //     println!("play {}", play);
       // }
       // println!("{}", line);
    
    }
}
