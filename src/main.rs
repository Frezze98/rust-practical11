use rand::Rng;

// Function to generate a random vector of length n with values [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Function to find the minimum adjacent sum in a vector and return the sum and the indices
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    if data.len() < 2 {
        panic!("The vector must contain at least two elements");
    }

    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index, min_index + 1)
}

// Function to display the vector and the minimum adjacent sum in a formatted manner
fn display_vector_info(data: &[i32]) {
    let (min_sum, index1, index2) = min_adjacent_sum(data);

    // Display indices
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Display data
    print!("data:    ");
    for &value in data {
        print!("{:>3},", value);
    }
    println!();

    // Display connection for minimum sum
    print!("indexes: ");
    for i in 0..data.len() {
        if i == index1 {
            print!("\\__");
        } else if i == index2 {
            print!("__/");
        } else {
            print!("    ");
        }
    }
    println!();

    // Display minimum adjacent sum
    println!(
        "min adjacent sum={}+{}={} at indexes:{}:{}",
        data[index1], data[index2], min_sum, index1, index2
    );
}

fn main() {
    // Generate a random vector of length 20
    let data = gen_random_vector(20);

    // Display the vector information
    display_vector_info(&data);
}
