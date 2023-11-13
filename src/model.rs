use std::collections::HashMap;

use crate::{sentence::Sentence, symbol::SymbolName};

pub type Model = HashMap<SymbolName, bool>;

fn check_all(
	knowledge: &impl Sentence,
	query: &impl Sentence,
	symbols: Vec<SymbolName>,
	model: Model,
) -> bool {
	if symbols.len() == 0 {
		return !knowledge.evaluate(&model) || query.evaluate(&model);
	}

	let mut remaining = symbols.clone();
	let proposition = remaining.pop().unwrap();

	let mut model_true = model.clone();
	model_true.insert(proposition.clone(), true);

	let mut model_false = model.clone();
	model_false.insert(proposition.clone(), false);

	return check_all(knowledge, query, remaining.clone(), model_true)
		&& check_all(knowledge, query, remaining.clone(), model_false);
}

pub fn model_check(knowledge: &impl Sentence, query: &impl Sentence) -> bool {
	let mut symbols = knowledge.symbols();

	for symbol in query.symbols() {
		if !symbols.contains(&symbol) {
			symbols.push(symbol);
		}
	}

	check_all(knowledge, query, symbols, HashMap::new())
}
