static BASE: u32 = 2;

//Works but did not realise from test input that there 
//would be 12 bits per line (test has only 4)
pub fn part_a(input: &str) -> u32 {

    let mut total_lines = 0;
    let mut bits_pop: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in input.trim().split('\n') {
        let mut number: u32 = 0;
        for i in 0..line.len() {
            number += line.chars()
                .nth(i)
                .unwrap()
                .to_digit(10)
                .unwrap() * BASE.pow(i as u32);
        }
        
        for i in 0..line.len() {
            if number & (1 << i ) != 0 {
                bits_pop[i] += 1;
            }
        }
        total_lines += 1;
    }

    let bits_vec: Vec<bool> = bits_pop.iter().map(|value| {
        if value * 2 >= total_lines {true} else {false}
        })
        .collect();

    let mut gamma_rate: u32 = 0;
    for i in 0..=11 {
        println!("{}", bits_vec[11 - i] as u32);
        gamma_rate += bits_vec[i] as u32 * BASE.pow((11 - i) as u32);
    }

    let mask = (1 << 12) - 1;

    gamma_rate as u32 * (!gamma_rate as u32 & mask)
}

pub fn part_b(input: &str) -> u32 {

    let mut total_lines = 0;
    let mut bits_pop: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in input.trim().split('\n') {
        let mut number: u32 = 0;
        for i in 0..line.len() {
            number += line.chars()
                .nth(i)
                .unwrap()
                .to_digit(10)
                .unwrap() * BASE.pow(i as u32);
        }
        
        for i in 0..line.len() {
            if number & (1 << i ) != 0 {
                bits_pop[i] += 1;
            }
        }
        total_lines += 1;
    }

    let bits_vec: Vec<bool> = bits_pop.iter().map(|value| {
        if value * 2 >= total_lines {true} else {false}
        })
        .collect();

    let mut gamma_rate: u32 = 0;
    for i in 0..=11 {
        println!("{}", bits_vec[11 - i] as u32);
        gamma_rate += bits_vec[i] as u32 * BASE.pow((11 - i) as u32);
    }

    let mask = (1 << 12) - 1;

    gamma_rate as u32 * (!gamma_rate as u32 & mask)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2035764);
    }
}
