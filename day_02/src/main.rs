

use std::{fs::File, io::Read};


fn has_couples(num: &str) -> bool {
    let num_len = num.len() / 2;
    num[..num_len] == num[num_len..]
}

fn has_multiples(num: &str) -> bool {
    
    let mut curr_idx_num = 1;

    'main: while curr_idx_num <= num.len() / 2_usize {
        
        if !num.len().is_multiple_of(curr_idx_num) { curr_idx_num += 1; continue }
        
        let mut curr_idx = curr_idx_num;
        let mut chunk = &num[..curr_idx_num];
        
        while curr_idx < num.len() {

            if chunk != &num[curr_idx..curr_idx+curr_idx_num] {
                curr_idx_num += 1;
                continue 'main;
            }

            chunk = &num[curr_idx..curr_idx+curr_idx_num];
            curr_idx += curr_idx_num;

        }
        
        return true;
        
    }

    false

}


fn main() {
    
    let mut buff_inp = String::new();
    let mut file_inp = File::open("input").expect("Could not open file");
    file_inp.read_to_string(&mut buff_inp).expect("Could not read file");

    let mut sum_nums_couples: u64 = 0;
    let mut sum_nums_multi: u64 = 0;


    for range in buff_inp.trim().split(',') {
        let (first, second) = range.split_once('-').expect("Input not correct");
        let (first, second): (u64, u64) = (first.parse().unwrap(), second.parse().unwrap());

        for num in first..second+1 {
            
            if has_couples(&num.to_string()) {
                sum_nums_couples += num;                
            }

            if has_multiples(&num.to_string()) {
                sum_nums_multi += num;
            }
        }

    }

    println!("Sum of duplicates numbers is: {sum_nums_couples}");
    println!("Sum of multiples numbers is: {sum_nums_multi}");


}
