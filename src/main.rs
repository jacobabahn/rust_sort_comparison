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
    // let mut vec_merge: Vec<i32> = Vec::new();

    println!("Enter a number of items for the insertion_sort. ");

    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let insertion_sort_size: i32 = input_line.trim().parse().expect("Input not an integer");

    // let mut input_line = String::new();
    // std::io::stdin()
    //     .read_line(&mut input_line)
    //     .expect("Failed to read line");
    // let merge_sort_size: i32 = input_line.trim().parse().expect("Input not an integer");

    let mut rng = rand::thread_rng();
    for _ in 1..insertion_sort_size {
        vec.push(rng.gen_range(0, 100));
    }

    let now = Instant::now();
    insertion_sort(&mut vec);
    let elapsed_insertion = now.elapsed().as_millis();

    println!(
        "It took Insertion Sort {} milliseconds to sort the vector.",
        elapsed_insertion
    );

    if !check_sort(&vec) {
        println!("Sorting didn't work correctly.")
    }
}
