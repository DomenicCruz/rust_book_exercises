use std::collections::HashMap;
/*
 *Given a list of integers, use a vector and return the mean (the average value), 
 *median (when sorted, the value in the middle position),
 *and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 */

fn main() {
    let list = [1, 44, 3, 0,  65, 22, 5, 8, 14, 25, 36, 4, 99, 0, 2];
    let vector = Vec::from(list);
    
    println!("List: {:?}", list);
    println!("Mean: {}", mean(&vector));
    println!("Median: {}", median(&vector));
    println!("Mode: {}", mode(&vector));
    
}

fn mean(vector: &Vec<i32>) -> i32 {
    let len = vector.len() as i32;
    let mut sum: i32 = 0;
    for item in vector {
        sum += item;
    }
    sum / len
}

fn median(vector: &Vec<i32>) -> i32 {
    let mut sorted_vec = vector.clone();
    sorted_vec.sort();
    let halfish = vector.len() / 2;
    sorted_vec[halfish]
}

fn mode(vector: &Vec<i32>) -> i32 {
    let mut uniq_count = HashMap::new() ;
    for item in vector {
        let count = uniq_count.entry(item).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut most_often = 0;
    for (item, occurs) in uniq_count{
        if occurs > most_often {
            mode = *item;
            most_often = occurs;
        }
    }
    mode
}
