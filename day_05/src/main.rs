
use std::{fs::File, io::Read};


fn first_answer(buff_inp: &str) -> u64 {

    let mut counter_good_prods: u64 = 0;

    let x: Vec<_> = buff_inp.lines().collect();
    let mut split = x.split(|l| l.trim().is_empty());

    let first_part = split.next().unwrap();
    let second_part = split.next().unwrap();

    let ranges: Vec<(u128, u128)> = first_part.iter().map(|x| {
        let mut split = x.split("-");
        (split.next().unwrap().parse::<u128>().unwrap(), split.next().unwrap().parse::<u128>().unwrap())
    }).collect();

    for id_prod in second_part.iter().map(|x| x.parse::<u128>().unwrap()) {

        for (range_low, range_high) in ranges.iter() {
            if *range_low <= id_prod && id_prod <= *range_high {
                counter_good_prods += 1;
                break;
            }
        }
    }

    counter_good_prods
}


fn find_last_non_greater(num: u128, vector: &[u128]) -> usize {
    for (i, &val) in vector.iter().enumerate() {
        if num <= val {
            return i;
        }
    }
    vector.len()
}


fn find_tot_nums_in_ranges(buff_inp: &str) -> u128 {

    let x: Vec<_> = buff_inp.lines().collect();
    let input: Vec<(u128, u128)> = x.split(|line| line.trim().is_empty())
        .next()
        .unwrap()
        .iter().map(|line| {
            let mut split = line.split("-");
            (
                split.next().unwrap().parse::<u128>().unwrap(),
                split.next().unwrap().parse::<u128>().unwrap()
            )
        })
        .collect();

    let mut number_products: u128 = 0;
    let mut result_ranges: Vec<u128> = Vec::new();

    for (low_range, high_range) in input {
        
        let idx_low = result_ranges.binary_search(&low_range).unwrap_or_else(|i| i);
        let idx_high = result_ranges.binary_search(&(high_range + 1)).unwrap_or_else(|i| i);
        
        result_ranges.splice(idx_low..idx_high, std::iter::empty());

        if idx_low.is_multiple_of(2) {

            result_ranges.insert(idx_low, low_range);

            if idx_high.is_multiple_of(2) {
                result_ranges.insert(idx_low + 1, high_range);
            }
        }
        else if idx_high.is_multiple_of(2) {
            result_ranges.insert(idx_low, high_range);
        }


    }


    for chunk in result_ranges.chunks(2) {
        number_products += chunk[1] + 1 - chunk[0];
    }

    number_products

}

fn main() {

    let mut buff_inp = String::new();
    let mut file = File::open("prova").expect("Could not open file");
    file.read_to_string(&mut buff_inp).expect("Could not read file");

    println!("The number of good products is: {}", first_answer(&buff_inp));
    println!("The number of elements in all ranges is: {}", find_tot_nums_in_ranges(&buff_inp));
}
