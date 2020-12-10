use std::env;
use std::fs;

fn solve1(input: &mut Vec<usize>) -> usize {
    input.push(0); //outlet is 0 joltages
    input.sort();
    assert!(input.len() > 1);
    let (mut dif1, mut dif3) = (0, 0);
    assert_eq!(input.windows(2).find(|x| {
        match x[1] - x[0] {
            0 => panic!("two equal adapters, ignore?"),
            1 => {
                dif1 += 1;
                false
            }
            2 => false,
            3 => {
                dif3 += 1;
                false
            }
            _ => true,
        }
    }), None);
    //your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
    dif3 += 1;
    dif1 * dif3
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // assuming all passwords are lower case
    let input = fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))?;
    let mut input = input
        .lines()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    println!("P1: {}", solve1(&mut input));
    Ok(())
}

#[test]
fn test1() {
    let mut input = vec![16,10,15,5,1,11,7,19,6,12,4];
    assert_eq!(solve1(&mut input), 7 * 5);

}

#[test]
fn test2() {
    let mut input = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3,];
    assert_eq!(solve1(&mut input), 22 * 10);
}
