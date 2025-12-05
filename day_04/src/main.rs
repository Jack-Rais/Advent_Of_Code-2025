
use std::{fs::File, io::Read};

fn get_accessible(string_lines: &str) -> (u32, String) {

    let mut count_accessible: u32 = 0;
    let lines: Vec<&str> = string_lines.trim().lines().collect();
    let mut result: Vec<String> = lines.iter().map(|x| x.to_string()).collect();


    for (m , line) in lines.iter().enumerate() {
        for (idx, _) in line.chars().enumerate().filter(|(_n, x)| *x != '.') {

            let start = if idx == 0 { 0 } else { idx - 1 };
            let end = (line.len() - 1).min(idx + 1);

            let mut tot_wrong: u8 = 0;

            if m != 0 {
                let line_up = lines.get(m - 1).unwrap();
                let count:u8 = line_up[start..end+1].chars().filter(|x| *x != '.').count().try_into().unwrap();

                tot_wrong += count;
            }

            if let Some(line_down) = lines.get(m + 1) {
                let count: u8 = line_down[start..end+1].chars().filter(|x| *x != '.').count().try_into().unwrap();

                tot_wrong += count;
            }

            tot_wrong += if idx == 0 { 0 } else {
                match line.chars().nth(idx - 1).unwrap() {
                    '@' => 1,
                    _ => 0
                }
            };

            tot_wrong += if idx == line.len() - 1 { 0 } else {
                match line.chars().nth(idx + 1).unwrap() {
                    '@' => 1,
                    _ => 0
                }
            };

            if tot_wrong < 4 {
                count_accessible += 1;
                result.get_mut(m).unwrap().replace_range(idx..idx+1, ".");
            }

        }

    }

    (count_accessible, result.join("\n"))

}


fn main() {
    
    let mut buff_inp = String::new();
    let mut file = File::open("input").expect("Could not open file");
    file.read_to_string(&mut buff_inp).expect("Could not read file");

    let count_accessible_once: u32 = get_accessible(&buff_inp).0;
    let mut count_accessible_total: u32 = 0;

    loop {
        let (num_acc, buff_new) = get_accessible(&buff_inp);

        count_accessible_total += num_acc;
        buff_inp = buff_new;

        if num_acc == 0 {
            break;
        }
    }

    println!("The number of papers removable the first time is: {count_accessible_once}");
    println!("The number of papers removable totally: {count_accessible_total}");
    
}
