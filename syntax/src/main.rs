use std::io;

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci (n - 2)
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let array: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return array;
}

fn main() {
    let res = fibonacci(10);
    let a : [i32; 5] = [10, 20, 30, 40, 50];
    for element in a {
        println!("Array a element = {element}");
    }    
    let mut b = take_vector();
    insertion_sort(&mut b);
    println!("Sorted b = {:?}", b);

    for element in b {
        println!("Array b element = {element}");
    }
    println!("Fibonacci(10) = {}", res);
    println!("Hello world!");

    let mut arr = [10, 20, 30, 40];
    let p = arr.as_mut_ptr();

    unsafe {
        *p.add(1) = 99;
    }

    println!("{:?}", arr);
}
