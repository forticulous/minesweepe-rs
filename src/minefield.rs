#[derive(Debug)]
pub struct Minefield<T> {
    pub num_rows: usize,
    pub num_cols: usize,
    data: Vec<T>
}

impl<T: Copy> Minefield<T> {
    pub fn new(num_rows: usize, num_cols: usize, initial_state: T) -> Minefield<T> {
        let mut data = Vec::with_capacity(num_rows * num_cols);
        for _ in 0..data.capacity() {
            data.push(initial_state.clone());
        }

        Minefield {
            num_rows: num_rows,
            num_cols: num_cols,
            data: data
        }
    }
    fn row_col_to_index(&self, row: usize, col: usize) -> usize {
        row * self.num_rows + col
    }

    pub fn get_at(&self, row: usize, col: usize) -> &T {
        let idx = self.row_col_to_index(row, col);

        &self.data[idx]
    }

    pub fn get_at_mut(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.row_col_to_index(row, col);

        &mut self.data[idx]
    }
}
