use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // assuming all passwords are lower case
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let input = input.lines().map(|x| x.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    for (index, &value) in (&input[25..]).iter().enumerate() {
        let input_slice = &input[index..index+25];
        let found = input_slice.iter().find(|&&x| {
            let other_value = if x > value {
                x - value
            } else {
                value - x
            };
            input_slice.contains(&other_value)
        });
        if found.is_none() {
            println!("P1 {}", value);
            break;
        }
    }
    Ok(())
}
