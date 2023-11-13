use std::collections::HashMap;

use crate::{merge_symbols, sentence::Sentence, symbol::SymbolName};

pub type Model = HashMap<SymbolName, bool>;

fn check_all(
	knowledge: &dyn Sentence,
	query: &dyn Sentence,
	symbols: Vec<SymbolName>,
	model: Model,
) -> bool {
	if symbols.len() == 0 {
		return !knowledge.evaluate(&model) || query.evaluate(&model);
	}

	let mut remaining = symbols.clone();
	let symbol = remaining.pop().unwrap();

	for value in [true, false] {
		let mut next_model = model.clone();
		next_model.insert(symbol.clone(), value);

		if !check_all(knowledge, query, remaining.clone(), next_model) {
			return false;
		}
	}

	true
}

pub fn model_check(knowledge: &dyn Sentence, query: &dyn Sentence) -> bool {
	check_all(
		knowledge,
		query,
		merge_symbols!(knowledge, query),
		HashMap::new(),
	)
}
