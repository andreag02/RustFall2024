fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    // Using Map and collect
    let doubled_with_map = process_vector_with_map(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x*2
    });

    let replaced_with_map = process_vector_with_map(numbers.clone(), |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2 {0} else {x}
    });

    // Using a for loop
    let doubled_with_for_loop = process_vector_with_for_loop(numbers.clone(), |x| {
        x*2
    });

    let replaced_with_for_loop = process_vector_with_for_loop(numbers, |x| {
        if x > 2 {0} else {x}
    });

    println!("Process vector using map and collect:");
    println!("Doubled: {:?}", doubled_with_map);
    println!("Replaced: {:?}", replaced_with_map);

    println!();

    println!("Process vector using a for loop:");
    println!("Doubled: {:?}", doubled_with_for_loop);
    println!("Replaced: {:?}", replaced_with_for_loop);
}