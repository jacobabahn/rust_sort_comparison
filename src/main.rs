use rand::Rng;
use std::time::Instant;

fn insertion_sort(vec: &mut Vec<i32>) {
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}

fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());
        let merged = merge(&left, &right);

        merged
    }
}

fn check_sort(vec: &Vec<i32>) -> bool {
    for i in 1..vec.len() - 1 {
        if vec[i] > vec[i + 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut vec_merge: Vec<i32> = Vec::new();

    println!("Enter a number of items for the insertion_sort. ");

    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let insertion_sort_size: i32 = input_line.trim().parse().expect("Input not an integer");

    println!("Enter a number of items for the merge_sort. ");

    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let merge_sort_size: i32 = input_line.trim().parse().expect("Input not an integer");

    let mut rng = rand::thread_rng();
    for _ in 1..insertion_sort_size {
        vec.push(rng.gen_range(0, 100));
    }

    for _ in 1..merge_sort_size {
        vec_merge.push(rng.gen_range(0, 100));
    }

    let insertion_now = Instant::now();
    insertion_sort(&mut vec);
    let elapsed_insertion = insertion_now.elapsed().as_millis();

    let merge_now = Instant::now();
    let merged = merge_sort(&vec_merge);
    let elapsed_merge = merge_now.elapsed().as_millis();

    println!(
        "It took Insertion Sort {} milliseconds to sort the vector.",
        elapsed_insertion
    );

    println!(
        "It took Merge Sort {} milliseconds to sort the vector.",
        elapsed_merge
    );

    if !check_sort(&vec) || !check_sort(&merged) {
        println!("Sorting didn't work correctly.")
    }
}
