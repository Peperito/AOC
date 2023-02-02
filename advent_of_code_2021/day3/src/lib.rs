
static BASE: u32 = 2;

//Works but ugly
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
        gamma_rate += bits_vec[i] as u32 * BASE.pow((11 - i) as u32);
    }

    let mask = (1 << 12) - 1;

    gamma_rate as u32 * (!gamma_rate as u32 & mask)
}


//Props to Jocelyn Stericker
pub fn rating(input: &str, most_common: bool) -> i64 {
    let mut lines: Vec<_> = input.trim().split('\n').collect();
    for d in 0usize..lines[0].len() {
        let mut ones = 0;
        let line_count = lines.len();
        for line in &lines {
            let c = line.chars().nth(d).unwrap();
            if c == '1' {
                ones += 1;
            }
        }

        let zeros = line_count - ones;

        let to_check = if most_common {
            if ones >= zeros {
                '1'
            } else {
                '0'
            }
        } else if ones >= zeros {
            '0'
        } else {
            '1'
        };

        lines = lines
            .into_iter()
            .filter(|s| s.chars().collect::<Vec<_>>()[d] == to_check)
            .collect();

        if lines.len() == 1 {
            return i64::from_str_radix(lines[0], 2).unwrap();
        }
    }

    panic!();
}

pub fn part_b(input: &str) -> i64 {
    rating(input, true) * rating(input, false)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_a() {
        assert_eq!(super::part_b(include_str!("input.txt")), 230);
    }
}
