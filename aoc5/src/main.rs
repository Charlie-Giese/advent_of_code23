use helper::lines_from_file;
use regex::Regex;

const PATH : &str = "./src/input.txt";

fn main() {

    problem1();
    problem2();

}

fn problem1() {

    let mut vec = lines_from_file(PATH);
    vec.retain(|x| !(x.is_empty()));
    
    let re = Regex::new(" ").unwrap();
    let seeds : Vec<i64> = re.split(&vec[0]).filter_map(|x| x.parse::<i64>().ok()).collect();

    vec.remove(0);

    let mut maps : Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut layer_map : Vec<(i64, i64, i64)> = vec![];


    for line in vec.iter().skip(1) {
        if line.chars().any(|x| x.is_numeric()) {
            let map : Vec<i64> = re.split(&line).filter_map(|x| x.parse::<i64>().ok()).collect();
            layer_map.push((map[0], map[1], map[2]));
        } else {
            maps.push(layer_map);
            layer_map = vec![];
        }
    }
    maps.push(layer_map);

    let mut locations : Vec<i64> = vec![];
    
    for seed in seeds.iter() {
        let mut temp = *seed;
        for range in maps.iter() {
            temp = lookup(temp, range);
        }
        locations.push(temp);
    }
    
    println!("Min Location Number = {:?}", locations.iter().min());
    


    //println!("{:?}", vec); 
}

fn problem2() {
    let mut vec = lines_from_file(PATH);
    vec.retain(|x| !(x.is_empty()));
    
    let re = Regex::new(" ").unwrap();
    let seed_ranges : Vec<i64> = re.split(&vec[0]).filter_map(|x| x.parse::<i64>().ok()).collect();

    vec.remove(0);

    let mut maps : Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut layer_map : Vec<(i64, i64, i64)> = vec![];


    for line in vec.iter().skip(1) {
        if line.chars().any(|x| x.is_numeric()) {
            let map : Vec<i64> = re.split(&line).filter_map(|x| x.parse::<i64>().ok()).collect();
            layer_map.push((map[0], map[1], map[2]));
        } else {
            maps.push(layer_map);
            layer_map = vec![];
        }
    }
    maps.push(layer_map);

    let mut seeds : Vec<(i64, i64)> = vec![];
    for n in 0..seed_ranges.len() / 2 {
        let start = seed_ranges[2 * n];
        let length = seed_ranges[2 * n + 1];
        seeds.push((start, length));
    }
    println!("{:?}", seeds);
    /*
    for seed in seeds.iter() {
        let mut temp = *seed;
        for range in maps.iter() {
            temp = lookup(temp, range);
        }
        locations.push(temp);
    }
    
    println!("Min Location Number = {:?}", locations.iter().min());*/
    
}


fn lookup(val : i64, maps : &Vec<(i64, i64, i64)>) -> i64 {

    let mut result : i64 = 0;
    

    for r in maps.iter() {
        let (dest, src, n) = (r.0, r.1, r.2);
        if val >= src && val <= src + n {
            result = val - src + dest;
            break;
        } else {
            result = val;
        }
    }
    result
}
