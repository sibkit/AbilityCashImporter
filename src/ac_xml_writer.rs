use std::fs::File;
use std::io::{self, Write};
use rust_decimal_macros::dec;
use xmltree::{Element, XMLNode};
use crate::ac_xml_template_build::{build_template, create_node_with_text};
use crate::aci::{AciError, box_err};

use crate::entry::Entry;
use crate::JResult;

fn get_element_by_name_mut<'b>(name: &str, parent: &'b mut Element) -> JResult<&'b mut Element> {
	let mut elements: Vec<&'b mut Element> = Vec::new();
	for n in (&mut parent.children).iter_mut() {
		let el_opt = n.as_mut_element();
		if el_opt.is_some() {
			let el = el_opt.unwrap();
			if el.name.eq(name) { elements.push(el); }
		}
	}
	if elements.len() != 1 { Err(box_err("transactions node not found!"))}
	else { Ok(elements.swap_remove(0)) }
}

pub fn write(entries: &Vec<Entry>, file_path: &str, account_name: &str) -> JResult<()> {
	let mut doc = build_template()?;
	let mut e_transactions = get_element_by_name_mut("transactions", &mut doc)?;

	for entry in entries {
		let mut e_transaction = Element::new("transaction");
		let mut n_date = create_node_with_text("date", &entry.date.format("%Y-%m-%dT%H:%M:%S").to_string());
		e_transaction.children.push(n_date);

		if entry.amount >= dec!(0) {
			let mut e_income = Element::new("income");
			e_income.children.push(XMLNode::Element(Element::new("executed")));
			let mut e_income_account = Element::new("income-account");
			e_income_account.children.push(create_node_with_text("name", account_name));
			e_income_account.children.push(create_node_with_text("currency", "RUR"));
			e_income.children.push(XMLNode::Element(e_income_account));
			e_income.children.push(create_node_with_text("income-amount", &entry.amount.to_string()));
			e_transaction.children.push(XMLNode::Element(e_income));
		} else {
			let mut e_expense = Element::new("expense");
			e_expense.children.push(XMLNode::Element(Element::new("executed")));
			let mut e_expense_account = Element::new("expense-account");
			e_expense_account.children.push(create_node_with_text("name", account_name));
			e_expense_account.children.push(create_node_with_text("currency", "RUR"));
			e_expense.children.push(XMLNode::Element(e_expense_account));
			e_expense.children.push(create_node_with_text("expense-amount", &entry.amount.to_string()));
			e_transaction.children.push(XMLNode::Element(e_expense));
		}
		e_transaction.children.push(
			create_node_with_text("comment", &format!("[{}] {}", &entry.agent, &entry.description.as_ref().unwrap_or(&"".to_string()))));

		let mut e_nec1 = Element::new("extra-comment");
		e_nec1.attributes.insert("n".to_string(), "1".to_string());
		e_nec1.children.push(XMLNode::Text(entry.document.as_ref().unwrap_or(&"".to_string()).to_string()));
		e_transaction.children.push(XMLNode::Element(e_nec1));

		let mut e_nec4 = Element::new("extra-comment");
		e_nec4.attributes.insert("n".to_string(), "4".to_string());
		e_nec4.children.push(XMLNode::Text(format!("Выписка {}", file_path)));
		e_transaction.children.push(XMLNode::Element(e_nec4));

		let mut n_transaction = XMLNode::Element(e_transaction);
		e_transactions.children.push(n_transaction);
	}
	doc.write(File::create(file_path)?)?;
	Ok(())
}