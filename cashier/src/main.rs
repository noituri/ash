use cash_file::CashFile;

mod cash_file;
mod compiler;

fn main() {
    let cash_file = CashFile::from_file("../test.cash").unwrap();
    cash_file.compile();
}
