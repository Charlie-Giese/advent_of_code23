use helper::*;
use regex::Regex;

const PATH : &str = "./src/input.txt";
const DIM : usize = 140;

fn main() {

    problem1();
}

fn find_adj(arr : [[char; DIM]; DIM], position : (i32, i32)) -> Vec::<(usize, usize)> {

    let mut result = Vec::<(usize, usize)>::new();

    for dx in -1 .. 2 {
        for dy in -1 .. 2 {

            let (newX, newY) : (i32, i32)= (position.0 + dx, position.1 + dy);

            if (newX < DIM as i32 && newX >= 0) && (newY < DIM as i32 && newY >= 0) && (dx, dy) != (0, 0) {
                result.push((newX as usize, newY as usize));
            }
        }
    }
    result
}



fn problem1() {

    let mut array  = [['.'; DIM]; DIM];

    if let Ok(lines) = read_lines(PATH) {
        for (i, line) in lines.enumerate() {
            let x = match line {
                Ok(val) => val,
                Err(error) => panic!("Could not read line ({:?})", error),
            };
            let characters = x.chars();
            for (j, c) in characters.enumerate() {
                array[i][j] = c;
            }
        } 
    }

    let mut matches = Vec::<(usize, usize)>::new();

    

    for i in 0 .. DIM {
        for j in 0 .. DIM {
            let mut val = array[i][j];
            if val.is_numeric() {
                let mut adj_ind = find_adj(array, (i.try_into().unwrap(), j.try_into().unwrap()));
                for idx in adj_ind.iter() {
                    if array[idx.0][idx.1] != '.' && !(array[idx.0][idx.1].is_numeric()) {
                        matches.push((i, j));
                    }
                }

            }
        }
    }

    let mut result = Vec::new();
    let mut indices = Vec::new();

    for tupl in matches.iter() {
        let mut num_string = String::new();
        
        let idx = tupl.0;
        let idy = tupl.1 as i32;

        for mut i in idy - 2 .. idy{
            while !(array[idx][i as usize].is_numeric()) {
                i += 1;
            }
            if (i >= 0 && i < DIM as i32) {
                if array[idx][i as usize].is_numeric() && !(indices.contains(&(idx, i))) {
                    num_string.push(array[idx][i as usize]);
                    indices.push((idx, i));
                }
            }
        }
        for i in idy .. idy + 3 {
            if !(array[idx][i as usize].is_numeric()) {
                break;
            }
            if (i >= 0 && i < DIM as i32) {
                if array[idx][i as usize].is_numeric() && !(indices.contains(&(idx, i))){
                    num_string.push(array[idx][i as usize]);
                    indices.push((idx, i));
                }
            }
        }

        if !(num_string.is_empty()) {
            let num = num_string.parse::<u32>().unwrap();
            if result.len() != 0 {
                if result[result.len() - 1] != num {
                    result.push(num);
                }
            } else if result.len() == 0 {
                result.push(num);
            }
        }
    }
    let sum : u32 = result.iter().sum();
    println!("Sum: {:?}", sum);
    

    
}
