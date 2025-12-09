
use std::{fs::File, io::Read};
use std::collections::{HashMap, HashSet};


fn first_answer(buff_inp: &str) -> u64 {

    let mut result: u64 = 0;

    let mut idxs: HashSet<usize> = HashSet::new();
    for line in buff_inp.lines() {
        for (n, element) in line.chars().enumerate() {

            if element == 'S' {
                idxs.insert(n);
            }
            else if element == '^' && idxs.contains(&n) {
                result += 1;

                idxs.remove(&n);
                idxs.insert(n + 1);
                idxs.insert(n - 1);
            }

        }
    }

    result
}

fn second_answer(buff_inp: &str) -> u64 {

    let mut count_lines: HashMap<usize, u64> = HashMap::new();

    for line in buff_inp.lines() {
        for (n, element) in line.chars().enumerate() {

            match element {
                'S' => {
                    *count_lines.entry(n).or_insert(0) += 1;
                },
                '^' => {
                    if let Some(&count) = count_lines.get(&n) {
                        *count_lines.entry(n + 1).or_insert(0) += count;
                        *count_lines.entry(n - 1).or_insert(0) += count;

                        count_lines.remove(&n);
                    }
                },
                _ => {}
            }

        }
    }

    let mut result: u64 = 0;
    for x in count_lines.values() { result += *x }

    result
}

fn main() {

    let mut file_inp = File::open("input").expect("Could not open file");
    let mut buff_inp = String::new();

    file_inp.read_to_string(&mut buff_inp).expect("Could not read file");

    println!("The tachyon split {} times", first_answer(&buff_inp));
    println!("The number of parallel universes is: {}", second_answer(&buff_inp));

}
