use helper::read_lines;

const PATH : &str = "./src/input.txt";

#[derive(Clone)]
struct Round {
    hand   : String,
    bid    : i32,
    score  : i32,
    rank   : i32,
}

fn main() {

    let mut rounds : Vec<Round> = vec![];
    
    if let Ok(lines) = read_lines(PATH) {
        for line in lines {
            let string = match line {
                Ok(val) => val,
                Err(error) => panic!("Could not read Input ({})", error),
            };
            let split = string.split(" ");
            let parts = split.collect::<Vec<&str>>();
            let mut round = Round {
                hand    : String::from(parts[0]),
                bid     : parts[1].parse::<i32>().unwrap(),
                score   : 0,
                rank    : 0,
            };
            
            round.score =  round.generate_score();
            rounds.push(round);
            
        }
    }
    for r in rounds.iter() {
        println!("{:?}", r.hand);
    }

    rounds.sort_by(|a, b| a.score.cmp(&b.score));
    for s in rounds[0].score .. rounds[rounds.len()-1].score + 1 {
        let indices = rounds.iter().enumerate().filter(|(_, &ref r)| r.score == s).map(|(index, _)| index).collect::<Vec<_>>();
        let mut equal_vals = vec![];
        for idx in indices.iter() {
            equal_vals.push(rounds[*idx].score);
        }

        println!("{:?} {:?}", indices, equal_vals);
    }
    
    
    



}

impl Round {


    fn count_matching(&self) -> Vec<(char, usize)> {

        let s = &self.hand;
        
        let mut classification : Vec<(char, usize)> = vec![];

        for c in s.chars() {
            classification.push((c, s.matches(c).count()));
        }
        classification
       
    }

    fn generate_score(&self) -> i32 {

        let mut score = 0;

        let classification = self.count_matching();

        let width = classification.iter().max_by_key(|x| x.1).unwrap();
        
        if width.1 == 1 {
            score = 2;
        } else if width.1 == 2 {
            if classification.iter().filter(|x| x.1 == 2).count() == 2 {
                score = 2;
            } else {
                score = 3;
            }
        } else if width.1 == 3 {
            if classification.iter().any(|x| x.1 == 2) {
                score = 5;
            } else {
                score = 4;
            }
        } else if width.1 == 4 {
            score = 6;
        } else if width.1 == 5 {
            score = 7;
        }
    
        score
    }
}

