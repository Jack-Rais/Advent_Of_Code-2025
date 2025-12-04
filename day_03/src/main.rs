
use std::{fs::File, io::Read};


fn find_max_num(n_res: usize, num: &str) -> String {

    let mut result: Vec<String> = vec!["0".to_string(); n_res];

    for (i, n) in num.chars().enumerate() {
        let start = if num.len() - i > n_res { 0 } else { n_res - (num.len() - i) };

        println!("{i}, {n}, {start}");

        for idx_num in start..result.len() {

            if result[idx_num].parse::<u8>().unwrap() < n.to_string().parse().unwrap() {
                result[idx_num] = n.to_string();
                result[idx_num+1..].fill("0".to_string());
                break;
            }
        }
    }

    result.join("")
}


fn main() {

    let mut buff_inp = String::new();
    let mut file = File::open("input").expect("Could not open file");
    file.read_to_string(&mut buff_inp).expect("Could not read to string");

    let mut sum_2: u64 = 0;
    let mut sum_12: u64 = 0;

    for line in buff_inp.trim().lines() {

        sum_2 += find_max_num(2, line).parse::<u64>().unwrap();
        sum_12 += find_max_num(12, line).parse::<u64>().unwrap();

    }

    println!("The sum of maximum values of 2 is {sum_2}");
    println!("The sum of maximum values of 12 is {sum_12}");

}
