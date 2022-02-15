fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub image1: String,
    pub image2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            image1: get_nth_arg(1),
            image2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}
