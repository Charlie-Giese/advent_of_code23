use helper::lines_from_file;
use regex::Regex;

const PATH : &str = "./src/input.txt";

fn main() {
    //problem1();
    problem2();
}

fn problem1() {

    let mut vec = lines_from_file(PATH); 
    
    let re = Regex::new(" ").unwrap();
    let times : Vec<i64> = re.split(&vec[0]).filter_map(|x| x.parse::<i64>().ok()).collect();
    let records : Vec<i64> = re.split(&vec[1]).filter_map(|x| x.parse::<i64>().ok()).collect();

    let mut result = vec![];
    
    for (time, record) in times.iter().zip(records.iter()) {
        let num = solve_race(*time, *record);
        result.push(num);
    }

    println!("{:?}", result.iter().product::<usize>());   
    
}

fn problem2() {

    let vec = lines_from_file(PATH); 
    
    let re = Regex::new(" ").unwrap();
    let times : Vec<i64> = re.split(&vec[0]).filter_map(|x| x.parse::<i64>().ok()).collect();
    let records : Vec<i64> = re.split(&vec[1]).filter_map(|x| x.parse::<i64>().ok()).collect();

    let mut time = String::new();
    let mut record = String::new();
    println!("{:?} {:?}", times, records);
    for (t,r) in times.iter().zip(records.iter()) {
        for c in t.to_string().chars() {
            time.push(c);
        }
        for c in r.to_string().chars() {
            record.push(c);
        }
    }
    println!("{:?} {:?}", time, record);
    let result = solve_race(time.parse::<i64>().unwrap(), record.parse::<i64>().unwrap());

    println!("{:?}", result);

    
}


fn solve_race(time : i64, record : i64) -> usize {

    let mut distance : Vec<i64> = vec![];

    for t in 0 .. time {
        let dist = (time - t) * t;
        if dist > record {
            distance.push(t);
        }
    }

    let num_ways = distance.len();

    num_ways
    
    
}