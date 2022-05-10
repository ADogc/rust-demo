use std::fmt;

pub fn cs() {
    let long_tuple = (1i32, 2u32);
    println!("{:?}", long_tuple.0);

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write!(f, "({},{})", self.0, self.1);
            write!(f, "({},{})", self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    // fn transpose(pair: Matrix) -> Matrix{
    //     let (a, b, c, d) = pair;
    //     (b, a, c, d)
    // } 

    // transpose(matrix)
}
