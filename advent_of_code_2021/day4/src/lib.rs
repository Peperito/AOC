#[derive(Debug, Default, Clone, Copy)]
pub struct Matrix {
    rows: [[u32; 5]; 5],
    bingo: bool,
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<u32> {
        Some(*self.rows.get(row)?.get(col)?)
    }

    pub fn set_at_index(&mut self, row: usize, col:usize, value: u32) {
        self.rows[row][col] = value
    }

    pub fn get_row_sum(&self, row: usize) -> u32 {
        let mut result = 0;
        for i in 0..self.rows.len() {
            result += self.rows[row][i];
        } 
        result
    }

    pub fn get_col_sum(&self, col: usize) -> u32 {        
        let mut result = 0;
        for i in 0..self.rows.len() {
            result += self.rows[i][col];
        } 
        result
    }

    pub fn get_result(&self, last_draw: u32) -> u32 {
        let mut total = 0;
        for i in 0..self.rows.len() {
            total += self.get_row_sum(i);
        }
        total * last_draw
    }

    pub fn get_index(&self, value: u32) -> Option<(usize, usize)> {
        for i in 0..self.rows.len() {
            for j in 0..self.rows.len() {
                if self.rows[i][j] == value {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

pub fn solution(input: &str, part: char) -> u32 {

    //build draws
    let mut draws: Vec<u32> = Vec::new();

    for line in input.trim().split("\n").take(1) {
        let numbers: Vec<&str> = line.split(',').collect(); 
        for numbers in numbers {
            let number: u32 = numbers.parse().unwrap();
            draws.push(number);
        } 
    }

    //build 5x5 grids
    let mut bingo_grids: Vec<Matrix> = Vec::new();
    let mut matrix = Matrix::default();
    let grids_input = input.trim().split("\n").skip(2);

    for (i, line) in grids_input.enumerate() {

        if i % 5 == 0 && i != 0 || line.is_empty() {
            bingo_grids.push(matrix.clone());
        }
        let numbers: Result<Vec<u32>, _> = line.split_whitespace()
            .map(|x| x.parse())
            .collect();
        for (j, number) in numbers.unwrap().iter().enumerate() {
            matrix.set_at_index(i % 5, j, *number);
        }
    }            

    //check winners
    let mut winner_scores: Vec<u32> = Vec::new();
    for draw in draws {
        for grid in &mut bingo_grids {
            match grid.get_index(draw) {
                Some((x,y)) => {
                    grid.set_at_index(x, y, 0);
                    if (grid.get_row_sum(x) == 0 
                    || grid.get_col_sum(y) == 0) 
                    && !grid.bingo {
                        grid.bingo = true;
                        if part == 'a'{
                            return grid.get_result(draw as u32);
                        }
                        if part == 'b' {
                            let score = grid.get_result(draw as u32);
                            winner_scores.push(score);
                        }
                    }
                },
                None => continue,
            }
        }
    }
    return *winner_scores.last().unwrap();
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::solution(include_str!("./../input.txt"), 'b'), 10478);
    }
}
