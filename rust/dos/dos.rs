use std::fs;

fn main() {
    let filepath = "./input.txt";

    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut grand_total = 0;
    for line in lines {
        //let mut round_score = 0;
        let mut plays = line.split(" ");
        let enemy_play = plays.next().unwrap();
        let my_play = plays.next().unwrap();
        let shape_score = match my_play {
            "X" => 1,
            "Y" => 2,
            _ => 3
        };
        let round_score = match enemy_play {
            // they play rock
            "A" => match my_play {
                "X" => 3,
                "Y" => 6,
                _ => 0,
            },
            // they play paper 
            "B" => match my_play {
                "X" => 0,
                "Y" => 3,
                _ => 6
            },
            // they play scissors
            _ =>  match my_play {
                "X" => 6,
                "Y" => 0,
                _ => 3
            }
        };
        let round_total = shape_score + round_score;
        grand_total += round_total;
       // for play in plays {
       //     println!("play {}", play);
       // }
       // println!("{}", line);
    
    }
    println!("grand total {}", grand_total);
}
