use cash_file::CashFile;

mod cash_file;
mod compiler;
mod scope;

fn main() {
    let cash_file = CashFile::from_file("../main.cash").unwrap();
    cash_file.compile();
}
