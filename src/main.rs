use std::io;

fn main() {
    println!("Hello, write an integer number: ");

    let mut integer_number = String::new();
    io::stdin().read_line(&mut integer_number).unwrap();

 
    let integer_number: u32 = integer_number
        .trim()
        .parse()
        .expect("Error: Write a valid integer number!");

    
    let binary_string: String = format!("{:b}", integer_number);


    let total_de_uns: usize = binary_string.chars()
        .filter(|c| *c == '1')
        .count();
    
    println!("A conversão em binário é: {}", binary_string);
    println!("E o total de digitos '1' são: {}", total_de_uns);
}
