pub fn part_a(input: &str) -> i64 {
    let mut a = 0;
    for line in input.trim().split('\n') {
        a += 1
    }
    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 3);
    }
}
