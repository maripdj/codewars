fn high_and_low(numbers: &str) -> String {
    let nums: Vec<i32> = numbers
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let max = *nums.iter().max().unwrap();
    let min = *nums.iter().min().unwrap();
    format!("{} {}", max, min)
}

fn main() {
    let high_and_low1 = "1 2 3 4 5";
    let high_and_low2 = "1 2 -3 4 5";
    let high_and_low3 = "1 9 3 4 -5";

    println!("{}", high_and_low(high_and_low1));
    println!("{}", high_and_low(high_and_low2));
    println!("{}", high_and_low(high_and_low3));
}
