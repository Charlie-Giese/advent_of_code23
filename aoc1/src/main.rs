use helper::read_lines;
use regex::Regex;

fn main() {

    problem1();
    problem2();
}

fn problem1() {
    

    let mut sum : u32 = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            let x = match line {
                Ok(val) => val,
                Err(error) => panic!("Problem opening the file: {:?}", error)
            };

            let mut nums = Vec::<char>::new();

            for c in x.chars() {
                if c.is_numeric() {
                    nums.push(c);
                }
            }
            
            let mut num_string = String::new();

            num_string.push(nums[0]);
            num_string.push(nums[nums.len() -1]);
         
            let num_val = num_string.parse::<u32>().unwrap();
            sum += num_val;
            
        }
    }

    println!("Sum = {:?}", sum);

}

fn problem2() {

    let mut sum : u32 = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            let x = match line {
                Ok(val) => val,
                Err(error) => panic!("Problem opening the file: {:?}", error)
            };

            let mut checklist = Vec::<(String, u32)>::new();

            checklist.push(("one".to_string(), 1));
            checklist.push(("two".to_string(), 2));
            checklist.push(("three".to_string(), 3));
            checklist.push(("four".to_string(), 4));
            checklist.push(("five".to_string(), 5));
            checklist.push(("six".to_string(), 6));
            checklist.push(("seven".to_string(), 7));
            checklist.push(("eight".to_string(), 8));
            checklist.push(("nine".to_string(), 9));

            let mut vec = Vec::<(u32, u32)>::new();
            
            for pattern in checklist {
                let (a, b) = pattern;
                let re = Regex::new(&a).unwrap();
                let vec_strings : Vec<String> = re.find_iter(&x).filter_map(|digits| digits.as_str().parse().ok()).collect();
                let captures : Vec<usize> = re.captures_iter(&x).map(|cap| cap.get(0).unwrap().start()).collect();

                for (_val, pos) in vec_strings.iter().zip(captures.iter()) {
                    vec.push((b, *pos as u32));
                }
            }

            for (i,c) in x.chars().enumerate() {
                if c.is_numeric() {
                    vec.push((c.to_digit(10).unwrap(), i as u32));
                }
            }

            vec.sort_by(|(_, k1), (_, k2)| k1.cmp(k2));
            
            let mut result = (vec[0].0).to_string();
            let digit_2 = (vec[vec.len() - 1].0).to_string();
            result.push_str(&digit_2);
            
            sum += result.parse::<u32>().unwrap();


        }
    }

    println!("Sum = {:?}", sum);

}

