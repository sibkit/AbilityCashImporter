use std::collections::HashMap;
use std::fs::File;
use chrono::{NaiveDate};
use crate::entry::Entry;
use csv::{ReaderBuilder, StringRecord};
use encoding_rs::{WINDOWS_1251};
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::{JResult};
use crate::aci::{box_err};

pub fn parse_decimal(text: &str) -> JResult<Decimal> {
	let s = text
		.replace(&['(', ')',' ','\"','\''], "")
		.replace(',',".");
	match Decimal::from_str_exact(&s) {
		Ok(val) => {Ok(val)}
		Err(e) => {Err(e.into())}
	}
}

fn get_dc_column_name<'a, 'b>(headers_map: &HashMap<String, usize>,rec: &'b StringRecord) -> JResult<&'b str>
{
	rec.get(*headers_map
		.get("d_c")
		.ok_or("d_c column not found")?)
		.ok_or(box_err("invalid record"))
}

fn get_agent(headers_map: &HashMap<String, usize>, rec: &StringRecord) -> Option<String> {
	let agent_caption = if get_dc_column_name(&headers_map, rec).ok()? == "C" { "plat_name" }
	else { "pol_name"};
	let agent_index = headers_map.get(agent_caption)
		.ok_or(box_err("column \"{}\" not found")).ok()?;
	let agent = rec.get(*agent_index).unwrap()
		.replace("Общество с ограниченной ответственностью", "ООО")
		.replace("ИНДИВИДУАЛЬНЫЙ ПРЕДПРИНИМАТЕЛЬ", "ИП")
		.replace("ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ", "ООО")
		.replace("ОБЩЕСТВА С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ", "ООО")
		.replace("Акционерное общество", "АО")
		.replace("АКЦИОНЕРНОЕ ОБЩЕСТВО", "АО")
		.replace("ТОРГОВЫЙ ДОМ", "ТД")
		.replace("Управление Федерального казначейства", "УФК")
		.replace("НАУЧНО ПРОИЗВОДСТВЕННОЕ ОБЪЕДИНЕНИЕ", "НПО")
		.replace("Закрытое акционерное общество","ЗАО")
		.replace("Индивидуальный предприниматель", "ИП");
	Some(agent)
}

pub fn read(file_name: &str) -> JResult<Vec<Entry>> {
	fn read_file(source_file_name: &str) -> JResult<DecodeReaderBytes<File, Vec<u8>>> {
		let file_result = File::open(source_file_name);
		let file = file_result?;
		let utf8_reader = DecodeReaderBytesBuilder::new()
			.encoding(Some(WINDOWS_1251))
			.build(file);
		Ok(utf8_reader)
	}

	let mut csv_reader = ReaderBuilder::new()
		.delimiter(b',')
		.delimiter(b'\t')
		.flexible(true)
		.has_headers(false)
		.from_reader(read_file(&file_name)?);

	let headers = csv_reader.records().next().ok_or("File is empty")??;
	let mut headers_map: HashMap<String, usize> = HashMap::default();

	for (i, header) in headers.iter().enumerate() {
		headers_map.insert(header.to_string(), i);
	}

	let rh = csv_reader.records().next()
		.ok_or(Box::new(std::fmt::Error::default()))??;

	if !rh.get(0)
		.ok_or(Box::new(std::fmt::Error::default()))?
		.contains("ID-выписки")
	{ return Err(box_err("File is empty")); }

	let calculate_amount = |rec: &StringRecord| -> JResult<Decimal> {
		let dc_index = *&headers_map.get("d_c").ok_or("column \"d_c\" not found")?;
		let dc_val = rec.get(*dc_index).ok_or("calculate_amount")?;
		let amount_index = *&headers_map.get("sum_val").ok_or("column \"sum_val\" not found")?;
		let amount_str = rec.get(*amount_index).ok_or("calculate amount error")?;
		let result = parse_decimal(amount_str);
		if dc_val == "C" { result } else {
			Ok(result? * dec!(-1))
		}
	};

	let rec_to_entry = |rec: Result<StringRecord, csv::Error>| -> JResult<Entry>
		{
			let rec = match rec {
				Ok(val) => { Ok(val) }
				Err(_) => { Err(box_err("rec error")) }
			};
			let rec = rec?;
			let e = Entry {
				agent: get_agent(&headers_map, &rec)
					.ok_or("Agent field err")?
					.to_string(),
				description: rec.get(headers_map["text70"])
					.map(str::to_string),
				document: Some(format!("Операция №{}", rec.get(headers_map["number"])
					.ok_or("\"number\" field problem")?)),
				amount: calculate_amount(&rec)?,
				date: NaiveDate::parse_from_str(rec
													.get(headers_map["date_oper"])
													.ok_or("\"date_oper\" field problem")?, "%d.%m.%Y")?
					.and_hms(0, 0, 0)
			};
			Ok(e)
		};
	csv_reader.records().into_iter().map(rec_to_entry).collect()
}

