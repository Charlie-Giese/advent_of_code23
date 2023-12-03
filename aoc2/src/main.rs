use helper::read_lines;
use regex::Regex;


fn main() {

    problem1();
    problem2()
}

fn problem1() {

    let key = ["red", "green", "blue"];
    let cypher = [12, 13, 14];

    let mut sum_total = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for (j, line) in lines.enumerate() {
            let string = match line {
                Ok(val) => val,
                Err(error) => panic!("Could not read line ({})", error),
            };

            let mut re = Regex::new(":").unwrap();
            let temp : Vec<&str> = re.split(&string).collect();
            
            re = Regex::new(";").unwrap();
            let fields : Vec<&str> = re.split(&temp[1]).collect();    

            let mut rule = 0;
            
            for field in fields {
                re = Regex::new(",").unwrap();
                let colors : Vec<&str> = re.split(&field).collect();
                //println!("{:?}", colors);
                
                for c in colors.iter() {
                    for (i, k) in key.iter().enumerate() {
                        if c.contains(k) {
                            for cap in Regex::new(r"\d{1,3}").unwrap().find_iter(c) {
                                let num_str = cap.as_str();
                                let num = num_str.parse::<i32>().ok().unwrap();

                                if num > cypher[i] {
                                    rule = 1;
                                }
                                
                            }

                        }   
                    }
                }                

            }
            if rule == 0 {
                sum_total += j + 1;
            }

        }
    }

    println!("Problem 1: {:?}", sum_total);
}

fn problem2() {

    let key = ["red", "green", "blue"];

    let mut sum_total = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            let string = match line {
                Ok(val) => val,
                Err(error) => panic!("Could not read line ({})", error),
            };

            let mut re = Regex::new(":").unwrap();
            let temp : Vec<&str> = re.split(&string).collect();
            
            re = Regex::new(";").unwrap();
            let fields : Vec<&str> = re.split(&temp[1]).collect();    

            let mut mins = vec![0; 3];
            
            for field in fields {
                re = Regex::new(",").unwrap();
                let colors : Vec<&str> = re.split(&field).collect();
                
                
                for c in colors.iter() {
                    for (i, k) in key.iter().enumerate() {
                        if c.contains(k) {
                            for cap in Regex::new(r"\d{1,3}").unwrap().find_iter(c) {
                                let num_str = cap.as_str();
                                let num = num_str.parse::<i32>().ok().unwrap();

                                if num > mins[i] {
                                    mins[i] = num;
                                }
                                
                            }

                        }   
                    }
                }                

            }
            let power = mins[0] * mins[1] * mins[2];
            sum_total += power;

        }
    }

    println!("Problem 2: {:?}", sum_total);
}