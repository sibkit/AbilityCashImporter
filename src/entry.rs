use std::fmt::{Display, Formatter};
use chrono::{ NaiveDateTime};
use rust_decimal::Decimal;

pub struct Entry{
	#[allow(dead_code)]
	pub(crate) agent: String,
	#[allow(dead_code)]
	pub(crate) description: Option<String>,
	#[allow(dead_code)]
	pub(crate) document: Option<String>,
	#[allow(dead_code)]
	pub(crate) amount: Decimal,
	#[allow(dead_code)]
	pub(crate) date: NaiveDateTime
}

impl Display for Entry {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f,"{} / {} / {}", self.date, self.agent, self.amount)
	}
}