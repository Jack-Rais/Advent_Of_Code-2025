
use std::{fs::File, io::Read};


fn first_answer(inp_buff: &str) -> u64 {
    
    let things: Vec<&str> = inp_buff.lines().collect();
    let operations: Vec<&str> = things.last().unwrap().split_whitespace().collect();
    let numbers: Vec<Vec<u64>> = things[0..things.len() - 1]
        .iter().map(|line| line.split_whitespace().map(|value| {
            value.to_string().parse::<u64>().unwrap()
        }).collect())
        .collect();

    let mut total: u64 = 0;

    for i in 0..operations.len() {
        
        let mut total_single = 0;
        match operations[i] {
            "*" => {
                total_single = 1;

                for x in numbers.iter() {
                    total_single *= x[i];
                }
                
            },
            "+" => {
                
                for x in numbers.iter() {
                    total_single += x[i];
                }
            },
            _ => println!("BOh")
        }

        total += total_single;

    }

    total
}

fn transpose(inp: Vec<&str>) -> Vec<Vec<String>> {
    
    let mut result = Vec::new();
    let inp: Vec<_> = inp.iter().map(|line| {line.chars().collect::<Vec<_>>()}).collect();

    for i in 0..inp[0].len() {
        let mut curr_vec = Vec::new();
        for elem in inp.iter() {
            curr_vec.push(elem[i].to_string());
        }
        result.push(curr_vec);
    }

    result
}

fn second_answer(buff_inp: &str) -> u64 {

    let mut tot: u64 = 0;
    let mut temporary_res: u64 = 0;
    let mut current_op: &str = "";
    let input = transpose(buff_inp.lines().collect::<Vec<&str>>().to_vec());


    for input_column in input.iter() {
        
        let (number, op) = (
            input_column.get(0..input_column.len() - 1).unwrap().join(""),
            input_column.last().unwrap()
        );

        if !op.trim().is_empty() {
            current_op = op.trim();

            match current_op {
                "*" => {
                    temporary_res = 1;
                },
                "+" => {
                    temporary_res = 0;
                },
                _ => panic!("Value of operation is wrong")
            }

        }

        if number.trim().is_empty() {

            tot += temporary_res;
            continue;

        }

        let current_number: u64 = number.trim().parse().unwrap();
        match current_op {
            "*" => {
                temporary_res *= current_number;
            },
            "+" => {
                temporary_res += current_number;
            },
            _ => panic!("Value of operation is wrong")
        }


    }

    tot += temporary_res;
    
    tot

}

fn main() {

    let mut file = File::open("input").expect("Could not open file");
    let mut inp_buff = String::new();

    file.read_to_string(&mut inp_buff).expect("Could not read file");

    println!("The total of results is: {}", first_answer(&inp_buff));
    println!("The right answer for the problem is: {}", second_answer(&inp_buff));
    
}
