use rand::Rng;

fn insertion_sort(vec: &mut Vec<i32>) {
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn check_sort(vec: Vec<i32>) -> bool {
    for i in 1..vec.len() {
        if vec[i] < vec[i + 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    // let mut vec_merge: Vec<i32> = Vec::new();

    print!("Enter a number of items for the insertion_sort. ");
    let mut line = String::new();
    let insertion_sort_size = std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", insertion_sort_size);

    // line = String::new();
    // print!("Enter a number of items for the merge_sort. ");
    // let merge_sort_size = std::io::stdin().read_line(&mut line).unwrap();

    let mut rng = rand::thread_rng();
    for i in 1..insertion_sort_size {
        vec.push(rng.gen_range(0, 100));
    }

    println!("{:?}", vec);

    insertion_sort(&mut vec);
}
