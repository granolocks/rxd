use rxd::{Reader, Runner};

fn main() {
    let mut reader = Reader::FileHandle(String::from("TODO get from env"));
    let runner = Runner::new(reader);
    runner.print_lines();
}
