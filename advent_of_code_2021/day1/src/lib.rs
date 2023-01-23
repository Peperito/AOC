pub fn part_a(input: &str) -> i64 {
    let mut depth_increases = 0;
    let measures_vector: Vec<u32> = input.trim()
            .split('\n')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
    for i in 1..measures_vector.len() {
        if measures_vector[i] > measures_vector[i - 1] {
            depth_increases += 1
        }
    }
    depth_increases
}

pub fn part_b(input: &str) -> i64 {
    let mut depth_increases = 0;
    let measures_vector: Vec<u32> = input.trim()
            .split('\n')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

    for i in 3..measures_vector.len() {
        if &measures_vector[i-2..=i].iter().sum::<u32>() 
        > &measures_vector[i-3..=i-1].iter().sum::<u32>() {
            depth_increases += 1;
        }
    }
    depth_increases
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1226);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 8);
    }
}
