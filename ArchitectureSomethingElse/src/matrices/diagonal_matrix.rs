use std::fs::File;
use std::io::Write;
use crate::matrices::base_matrix::BaseMatrix;

pub struct DiagonalMatrix {
    pub size: usize,
    pub diag: Vec<f64>,
}

impl BaseMatrix for DiagonalMatrix {
    fn input(&mut self, mut file: &mut File) {
        todo!()
    }

    fn random_input(&mut self) {
        todo!()
    }

    fn output(&self, mut file: &mut File) {
        file.write_all(format!("Diagonal matrix with size of {}:", self.size).as_bytes());

        for i in 0..self.size {
            for j in 0..self.size {
                if i == j {
                    file.write_all(self.diag[i].to_string().as_bytes());
                } else {
                    file.write_all("0 ".as_bytes());
                }
            }
            file.write_all("\n".as_bytes());
        }
    }

    fn get_average(&self) -> f64 {
        let mut sum: f64 = 0_f64;
        for i in 0..self.size {
            sum += self.diag[i];
        }
        return sum / self.size as f64 / self.size as f64;
    }
}
