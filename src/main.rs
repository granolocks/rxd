use rxd::runner::{Runner};

fn main() {
    let file_path = std::env::args().nth(1);
    let runner = Runner::new(file_path);
    runner.print_lines();
}
