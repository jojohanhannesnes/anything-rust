use std::collections::HashMap;

fn main() {
    let lists_of_values = vec![1,1,2,3,3,3];
    let median_result = median(&lists_of_values);
    println!("{median_result}");
    let mode_result = mode(&lists_of_values);
    println!("{mode_result}");
}

fn median(data: &Vec<i32>) -> i32 {
    let mut clone_data = data.clone();
    clone_data.sort();
    let middle = clone_data[clone_data.len() / 2];
    if clone_data.len() % 2 == 0 {
        (middle + clone_data[((clone_data.len()) / 2) - 1]) / 2
    } else {
        clone_data[clone_data.len() / 2]
    }
}

fn mode(data: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut lib = HashMap::new();
    for val in data {
       lib.entry(*val).and_modify(|v| *v += 1).or_insert(1);
    };
    println!("{:?}", lib);
    for (k,v) in lib {
        if v > result {
            result = k;
        }
    }
    result
}