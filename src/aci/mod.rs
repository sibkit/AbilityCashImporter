use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type JResult<T> = Result<T,Box<dyn Error>>;

#[derive(Debug)]
pub struct AciError {
	msg: String,
	#[allow(dead_code)]
	inner_error: Option<Box<dyn Error>>
}

impl Display for AciError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.msg.as_str())
	}
}

impl Error for AciError {}

pub fn box_err(msg: &str) -> Box<dyn Error> {
	Box::new(AciError { msg: msg.to_string(), inner_error: None })
}