use std::{ops, fmt::Display};



pub struct GenMatrix<T> {
    dim: (usize, usize),
    data: Vec<T>
}

impl <T> Display for GenMatrix<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for row in 0..self.dim.0 {
            write!(f, "\t[")?;

            for col in 0..self.dim.1 - 1 {
                write!(f, "{},", self.data[row * self.dim.1 + col])?;
            }

            for col in self.dim.1 - 1..self.dim.1 {
                write!(f, "{}", self.data[row * self.dim.1 + col])?;
            }
            write!(f, "]\n")?;
        }

        writeln!(f,"]")?;
        Ok(())
    }
}

    /** TODO:
 * 
 * - update without clone (use functions that take ownership)
 * - full index operations m[:][2] = [1, 2, 3, 4]; 
 * - extend to N dims
 * 
 **/

// Attempt of generalizing 2D matrix for any type.
impl <T> ops::Index<usize> for GenMatrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        
        return &self.data[row * self.dim.1..(row + 1) * self.dim.1];
    }
}

impl <T>  GenMatrix<T> {

    pub fn from_data(dim: (usize, usize), init_data: Vec<T>) -> Self {
        return Self{dim: dim, data: init_data};
    }

    pub fn from_default(dim: (usize, usize), default_value: T) -> Self
    where T: Clone {
        return Self{dim, data: vec![default_value; dim.0 * dim.1]};
    }

    pub fn dim(&self) -> (usize, usize) {return self.dim;}

    pub fn set_element(&mut self, indices: (usize, usize), e: T) {
        if indices.0 >= self.dim.0 || indices.1 >= self.dim.1 {
            panic!("Index error. Try to set element of matrix  with dim {:?} at indices {:?}", self.dim, indices);
        }
        self.data[indices.0 * self.dim.1 + indices.1] = e;
    }

    pub fn set_row(&mut self, row_index: usize, row: &[T]) where T: Clone {
        if row_index >= self.dim.0 {
            panic!("Index error. Try to set row of matrix with dim {:?} at row {:?}", self.dim, row_index);
        }
        if row.len() != self.dim.1 {
            panic!("Index error. Try to update a row of matrix with dim {:?} with a row of len {}", self.dim, row.len());
        }
        self.data.splice(row_index * self.dim.1..(row_index + 1) * self.dim.1, row.iter().cloned());
    }

    pub fn set_col(&mut self, col_index: usize, col: &[T]) where T: Clone {
        if col_index >= self.dim.1 {
            panic!("Index error. Try to set col of matrix with dim {:?} at col {:?}", self.dim, col_index);
        }
        if col.len() != self.dim.0 {
            panic!("Index error. Try to update a col of matrix with dim {:?} with a col of len {}", self.dim, col.len());
        }
        (0..self.dim.0).for_each(|row| {self.data[row * self.dim.1 + col_index] = col[row].clone();});
    }

    pub fn insert_row(&mut self, row_index: usize, row: &[T]) where T: Clone {
        if row.len() != self.dim.1 {
            panic!("Index error. Try to insert matrix with dim {:?} with row of size {}", self.dim, row.len());
        }
        let data_index = row_index * self.dim.1;
        self.data.splice(data_index..data_index, row.iter().cloned());
        self.dim.0 += 1;
    }

    pub fn append_row(&mut self, row: &[T]) where T: Clone {
        if row.len() != self.dim.1 {
            panic!("Index error. Try to append matrix with dim {:?} with row of size {}", self.dim, row.len());
        }
        self.data.extend(row.iter().cloned());
        self.dim.0 += 1;
    }

    pub fn remove_row(&mut self, row_index: usize) {
        if row_index > self.dim.0 - 1 {
            panic!("Index error. Try to remove from matrix with dim {:?} row {}", self.dim, row_index);
        }
        (0..self.dim.1).for_each(|_| { self.data.remove(row_index * self.dim.1); });
        self.dim.0 -= 1;
    }

    pub fn insert_col(&mut self, col_index: usize, col: &[T]) where T: Clone {
        if col.len() != self.dim.0 {
            panic!("Index error. Try to insert matrix with dim {:?} with col of size {}", self.dim, col.len());
        }
        self.dim.1 += 1;
        (0..self.dim.0).for_each(|row| {
            self.data.insert(row * self.dim.1 + col_index, col[row].clone());
        });
    }

    pub fn append_col(&mut self, col: &[T]) where T: Clone {
        if col.len() != self.dim.0 {
            panic!("Index error. Try to append matrix with dim {:?} with col of size {}", self.dim, col.len());
        }
        self.dim.1 += 1;
        (0..self.dim.0).for_each(|row| self.data.insert((row + 1) * self.dim.1 - 1, col[row].clone()));
    }

    pub fn remove_col(&mut self, col_index: usize) {
        if col_index > self.dim.1 - 1 {
            panic!("Index error. Try to remove from matrix with dim {:?} col {}", self.dim, col_index);
        }
        self.dim.1 -= 1;
        (0..self.dim.0).for_each(|row| { self.data.remove(row * self.dim.1 + col_index); });
    }
}