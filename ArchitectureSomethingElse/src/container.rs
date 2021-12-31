pub mod container {
    use std::fs::File;
    use std::io::Write;
    use rand::Rng;
    use rand::rngs::ThreadRng;
    use crate::matrices::base_matrix::BaseMatrix;
    use crate::matrices::matrix;
    use crate::matrices::diagonal_matrix;
    use crate::matrices::diagonal_matrix::DiagonalMatrix;
    use crate::matrices::lower_triangular_matrix;
    use crate::matrices::lower_triangular_matrix::LowerTriangularMatrix;
    use crate::matrices::matrix::Matrix;

    pub struct Container {
        pub size: usize,
        pub matrs: Vec<Box<dyn BaseMatrix>>,
    }

    pub trait ContainerInterface {
        fn input(&mut self, file: &mut File);
        fn random_input(&mut self);
        fn output(&self, file: &mut File);
        fn sort(&mut self);
    }

    impl ContainerInterface for Container {
        fn input(&mut self, file: &mut File) {
            todo!()
        }

        fn random_input(&mut self) {
            for i in 0..self.size {
                let b: Box<dyn BaseMatrix>;
                let mut range: ThreadRng = rand::thread_rng();
                let rand_num: i32 = range.gen();

                if rand_num % 3 == 0 {
                    b = Box::new(Matrix{size: 0, matr: Vec::new()});
                } else if rand_num % 3 == 1 {
                    b = Box::new(DiagonalMatrix{size: 0, diag: Vec::new()});
                } else {
                    b = Box::new(LowerTriangularMatrix{size: 0, elems_count: 0, elems: Vec::new()});
                }

                self.matrs.push(b);
                self.matrs[i].random_input();
            }
        }

        fn output(&self, mut file: &mut File) {
            file.write_all(format!("Number of matrices: {}\n", self.size).as_bytes());
            for i in 0..self.size {
                self.matrs[i].output(&mut file);
            }
        }

        fn sort(&mut self) {
            for i in 0..self.size - 1 {
                let mut min_ind: usize = i;

                for j in i + 1..self.size {
                    if self.matrs[j].get_average() <
                        self.matrs[min_ind].get_average() {
                        min_ind = j;
                    }
                }

                self.matrs.swap(min_ind, i);
            }
        }
    }
}