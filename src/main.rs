use std::error::Error;
use crate::aci::JResult;
use crate::alfa_bank_csv_reader::read;

mod entry;
mod alfa_bank_csv_reader;
mod aci;
mod ac_xml_writer;

fn main()
{
    let result = launch();
    if result.is_err() { handle_error(result.err().unwrap())};
}

fn handle_error(err: Box<dyn Error>)
{
    println!("{}", err);
}

fn launch()->JResult<()>
{
    let path = "D:\\Ledmaster\\Финансы\\Выписки банка\\Ледмастер (Альфа)\\Ледмастер альфа 2022.08.26.csv";
    //let mut ab_reader = AlfaBankCsvReader::new(path, "Альфа (LM)")?;
    let result = read(path)?;
    //if result.is_err() {
      //  println!("{:?}", result.as_ref().err().unwrap());
    //}
    println!("-----");
    //let result = ab_reader.read();

    for entry in result {
        println!("{}", entry);
    }

    Ok(())
}