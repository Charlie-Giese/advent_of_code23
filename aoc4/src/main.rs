use helper::read_lines;
use regex::Regex;

const PATH : &str = "./src/input.txt";

fn main() {

    let mut card_id = vec![];
    let mut wins_per_card = vec![];
    
    if let Ok(lines) = read_lines(PATH) {
        for line in lines {
            let card = match line {
                Ok(val) => val,
                Err(error) => panic!("Could not read card ({})", error),
            };

            let mut subbsets = vec![];

            let re = Regex::new(r":|\|").unwrap();
            for part in re.split(&card) {
                subbsets.push(part);
            }

            let card : Vec<&str> = subbsets[0].split(" ").collect();
            card_id.push(card[card.len() -  1].to_string().parse::<i32>().unwrap());

            let winning : Vec<&str> = subbsets[1].split(" ").collect();
            let given : Vec<&str> = subbsets[2].split(" ").collect();

            let mut ctr = 0;
            for x in given.iter() {
                for y in winning.iter() {
                    let _x = match (x.to_string().parse::<i32>(), y.to_string().parse::<i32>()) {
                        (Ok(x0), Ok(y0)) => {
                            if x0 == y0 {
                                ctr +=1;
                            }
                        }
                        (Ok(_), Err(_)) => continue,
                        (Err(_), Ok(_)) => continue,
                        (Err(_), Err(_)) => continue,
                    };
                   
                }
            }
            wins_per_card.push(ctr);
        }
    }
    let mut total_cards = 0;
    let mut extra_cards = vec![];
    for (i, card) in card_id.iter().enumerate() {
        let original_wins = wins_per_card[i];
        for w in 1..original_wins+1 {
            total_cards += 1;
            extra_cards.push(card+w);
        }
        let mut temp = vec![];
        for extra in extra_cards.iter_mut() {
            if extra == card {
                for w in 1..original_wins+1 {
                    total_cards += 1;
                    temp.push(card+w);
                }
            }
        }
        extra_cards.retain(|&x| x != *card);//remove(extra_cards.iter().position(|x| *x == *card).unwrap());
        for t in temp.iter() {
            extra_cards.push(*t);
        }
    }
    total_cards += card_id.len();

    println!("Total Cards {:?}", total_cards);

}

fn problem1() {
    let mut score = 0;
    
    if let Ok(lines) = read_lines(PATH) {
        for line in lines {
            let card = match line {
                Ok(val) => val,
                Err(error) => panic!("Could not read card ({})", error),
            };

            let mut subbsets = vec![];

            let re = Regex::new(r":|\|").unwrap();
            for part in re.split(&card) {
                subbsets.push(part);
            }

            let _card : Vec<&str> = subbsets[0].split(" ").collect();
            let winning : Vec<&str> = subbsets[1].split(" ").collect();
            let given : Vec<&str> = subbsets[2].split(" ").collect();

            let mut ctr = 0;
            println!("{:?} {:?}", winning, given);
            for x in given.iter() {
                for y in winning.iter() {
                    let _x = match (x.to_string().parse::<i32>(), y.to_string().parse::<i32>()) {
                        (Ok(x0), Ok(y0)) => {
                            if x0 == y0 {
                                ctr +=1;
                            }
                        }
                        (Ok(_), Err(_)) => continue,
                        (Err(_), Ok(_)) => continue,
                        (Err(_), Err(_)) => continue,
                    };
                   
                }
            }
            let base : i32 = 2;
            if ctr > 0 {
                score += 1 * base.pow(ctr - 1);
            }
            
            

        }
    }

    println!("Score: {:?}", score);
}