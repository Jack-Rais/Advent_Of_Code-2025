
use std::{fs::File, io::Read};

fn main() {
    
    let mut file = File::open("input").unwrap();
    let mut buff_file = String::new();
    file.read_to_string(&mut buff_file).unwrap();
    let actions = buff_file.trim().split("\n");

    let mut dial_num: i32 = 50;
    let mut num_zeros_precise: u32 = 0;
    let mut num_zeros_total: u32 = 0;

    for action in actions {

        let rotations: i32 = action.get(1..).unwrap().parse().unwrap();

        match action.chars().next().unwrap() {
            'R' => {
                if dial_num + (rotations % 100) >= 100 {
                    num_zeros_total += 1;
                }
                num_zeros_total += (rotations as f32 / 100.0).floor() as u32;
                dial_num = (dial_num + rotations) % 100;
            },
            'L' => {
                if dial_num - (rotations % 100) <= 0 && dial_num != 0 {
                    num_zeros_total += 1;
                }
                num_zeros_total += (rotations as f32 / 100.0).floor() as u32;
                dial_num = ((dial_num - rotations) % 100 + 100) % 100;
            },
            _ => panic!("Not an L or R")
        }

        if dial_num == 0 {
            num_zeros_precise += 1;
        }
        
    }

    println!("Dial now is in: {}", dial_num);
    println!("Stopped on 0 '{}' times", num_zeros_precise);
    println!("Met 0 '{}' times", num_zeros_total);

}
