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
        let outcome = plays.next().unwrap();
        let round_score = match outcome {
            "X" => 0,
            "Y" => 3,
            _ => 6
        };
        let shape_score = match enemy_play {
            // they play rock
            "A" => match outcome {
                "X" => 3,
                "Y" => 1,
                _ => 2,
            },
            // they play paper 
            "B" => match outcome {
                "X" => 1,
                "Y" => 2,
                _ => 3
            },
            // they play scissors
            _ =>  match outcome {
                "X" => 2,
                "Y" => 3,
                _ => 1
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
