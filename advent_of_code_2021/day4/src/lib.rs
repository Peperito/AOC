#[derive(Debug, Default)]
pub struct Matrix {
    rows: [[u32; 5]; 5],
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<u32> {
        Some(*self.rows.get(row)?.get(col)?)
    }
    pub fn push_at_index(&mut self, row: usize, col:usize, value: u32) {
        self.rows[row][col] = value
    }
}

pub fn part_a(input: &str) -> i64 {

    let mut draws: Vec<u32> = Vec::new();
    //change take for real input
    for line in input.trim().split("\n").take(1) {
        let numbers: Vec<&str> = line.split(',').collect(); 
        for numbers in numbers {
            let number: u32 = numbers.parse().unwrap();
            draws.push(number);
        } 
    }

    //change skip for real input
    let mut bingo_grids: Vec<&Matrix> = Vec::new();
    let mut matrix = Matrix::default();

    for (i, line) in input.trim().split("\n").skip(2).enumerate() {
        if line.len() <= 1 {
            continue
        }
        println!("{line}");
        let numbers: Result<Vec<u32>, _> = line.split_whitespace().map(|x| x.parse()).collect();

        for (j, number) in numbers.unwrap().iter().enumerate() {
            matrix.push_at_index(i % 5, j, *number);
        }
        if i % 5 == 4 {
            bingo_grids.push(&matrix);
        }
    }
    println!("{:?}", bingo_grids);
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("./../testinput.txt")), 1);
    }
}
