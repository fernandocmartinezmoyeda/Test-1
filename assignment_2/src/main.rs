// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers: [i32; 10] = [12, 25, 30, 7, 9, 14, 15, 22, 33, 40];

    println!("Analyzing numbers:");

    // Iterate through the array using a for loop
    for &num in numbers.iter() {
        // Check if the number is even or odd
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        // Check divisibility by 3, 5, and both
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    // Sum of all numbers in the array using a while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("\nThe sum of all numbers is: {}", sum);

    // Finding the largest number in the array using a loop
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}