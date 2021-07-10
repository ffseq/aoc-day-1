
use std::fs;

fn main() {
    println!("Running!");
    
    let numbers = parse_input("input.txt");

    let (a, b, c, abc) = find_nums(&numbers);
    println!("Ans: {} * {} * {} is {}", a, b, c, abc);
}

fn parse_input(inputfile: &str) -> Vec<i32> {
    let input_string = fs::read_to_string(inputfile)
        .expect("File not found!");

    let input_vec = input_string.lines().collect::<Vec<&str>>();
    let mut numbers: Vec<i32> = Vec::new();
    for elt in input_vec {
        println!("Number {}", elt);
        let num: i32 = elt.parse().unwrap();
        numbers.push(num);
    }
    
    
    numbers
}    

fn find_nums(v: &Vec<i32>) -> (i32, i32, i32, i32) {
    const TARGET: i32 = 2020;

    for i in 0..v.len()  {
        for j in i..v.len() {
            for k in j..v.len() {
                if v[i] + v[j] + v[k] == TARGET {
                    return (v[i], v[j], v[k], v[i] * v[j] * v[k]);
                }
            }
        }
    }

    (0, 0, 0, 0)
}
