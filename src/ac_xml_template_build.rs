use chrono::{Local, NaiveDateTime};
use xmltree::{Element, XMLNode};
use crate::aci::JResult;

fn create_element_with_text(name: &str, text: &str) ->  Element {
	let mut element = Element::new(name);
	element.children.push(XMLNode::Text(text.to_string()));
	element
}

pub fn create_node_with_text(name: &str, text: &str) -> XMLNode {
	let mut element = Element::new(name);
	element.children.push(XMLNode::Text(text.to_string()));
	XMLNode::Element(element)
}

pub fn build_template() ->JResult<Element> {
	let mut e_ability_cash = Element::new("ability-cash");

	let mut e_export_options = Element::new("export-options");

	e_export_options.children.push(create_node_with_text("module", "{05150516-3C4E-4A40-8EB3-B42C9DCE9E79}"));
	e_export_options.children.push(create_node_with_text("program-build", "267"));

	let res = Local::now().naive_local().format("%Y-%m-%dT%H:%M:%S");
	let mut e_export_date = create_element_with_text("export-date",&res.to_string());
	e_export_options.children.push(XMLNode::Element(e_export_date));
	e_export_options.children.push(XMLNode::Element(Element::new("account-folders")));
	e_export_options.children.push(XMLNode::Element(Element::new("time")));

	let mut ec_1 = create_element_with_text("extra-comment","Документ");
	ec_1.attributes.insert("n".to_string(),"1".to_string());
	e_export_options.children.push(XMLNode::Element(ec_1));

	let mut ec_2 = create_element_with_text("extra-comment","Контрагент");
	ec_2.attributes.insert("n".to_string(),"2".to_string());
	e_export_options.children.push(XMLNode::Element(ec_2));

	let mut ec_4 = create_element_with_text("extra-comment","Источник");
	ec_4.attributes.insert("n".to_string(),"4".to_string());
	e_export_options.children.push(XMLNode::Element(ec_4));
	e_ability_cash.children.push(XMLNode::Element(e_export_options));

	let mut e_currencies = Element::new("currencies");
	let mut e_rub_cur = Element::new("currency");
	e_rub_cur.attributes.insert("oid".to_string(),"{73430F56-F7AF-44ED-84F4-2411B86D925A}".to_string());
	e_rub_cur.attributes.insert("changed-at".to_string(),"2019-05-24T05:59:25".to_string());
	e_rub_cur.children.push(create_node_with_text("name","Российские рубли"));
	e_rub_cur.children.push(create_node_with_text("code","RUR"));
	e_rub_cur.children.push(create_node_with_text("precision","2"));

	e_currencies.children.push(XMLNode::Element(e_rub_cur));
	e_ability_cash.children.push(XMLNode::Element(e_currencies));

	let mut e_accounts = Element::new("accounts");
	let mut e_account = Element::new("account");
	e_account.children.push(create_node_with_text("name", "account_name"));
	e_account.children.push(create_node_with_text("currency", "RUR"));
	e_account.children.push(create_node_with_text("init-balance", "0"));
	e_accounts.children.push(XMLNode::Element(e_account));
	e_ability_cash.children.push(XMLNode::Element(e_accounts));

	e_ability_cash.children.push(XMLNode::Element(Element::new("transactions")));

	Ok(e_ability_cash)
}