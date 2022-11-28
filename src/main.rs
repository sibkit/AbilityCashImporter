use std::error::Error;
use std::fs::File;
use crate::ac_xml_template_build::build_template;
use crate::ac_xml_writer::write;
use crate::aci::JResult;
use crate::alfa_bank_csv_reader::read;

mod entry;
mod alfa_bank_csv_reader;
mod aci;
mod ac_xml_writer;
mod ac_xml_template_build;

fn main()
{
    let result = launch();
    if result.is_err() { handle_error(result.err().unwrap())};
}

fn handle_error(err: Box<dyn Error>)
{
    println!("{}", err);
}

fn convert_file(account: &str, source_file_path: &str, target_file_path: &str) -> JResult<()> {
    let entries = read(source_file_path)?;
    println!("Readed {} entries from {}", entries.len(),account);
    write(&entries, target_file_path, account)?;
    Ok(())
}

fn convert_LMA_LMF(date_prefix: &str) -> JResult<()> {
    convert_file("Альфа (LM)",
                 &format!("D:\\Ledmaster\\Финансы\\Выписки банка\\Ледмастер (Альфа)\\Ледмастер альфа {}.csv",date_prefix),
                 &format!("D:\\Ledmaster\\Финансы\\AC_import\\w_lm_alfa_{}.xml",date_prefix))?;
    convert_file("Альфа (ИП Ф)",
                 &format!("D:\\Ledmaster\\Финансы\\Выписки банка\\ИП Фролов (Альфа)\\ИП Фролов альфа {}.csv",date_prefix),
                 &format!("D:\\Ledmaster\\Финансы\\AC_import\\w_ipf_alfa_{}.xml",date_prefix))?;
    Ok(())
}

fn launch()->JResult<()>
{
    match convert_LMA_LMF("2022.10.27") {
        Ok(_) => Ok(()),
        Err(err) => {eprintln!("{}",err); Err(err)}
    }
        /*
    let path = "D:\\Ledmaster\\Финансы\\Выписки банка\\Ледмастер (Альфа)\\Ледмастер альфа 2022.08.26.csv";

    let entries = read(path)?;

    println!("-----");
    //let result = ab_reader.read();

    for entry in &entries {
        println!("{}", entry);
    }
    write(&entries, "write-text.xml", "Альфа (LM)")?;
    Ok(())*/
}