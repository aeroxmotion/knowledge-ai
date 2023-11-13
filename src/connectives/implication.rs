use crate::{model::Model, sentence::Sentence, symbol::SymbolName};

pub struct Implication(pub Box<dyn Sentence>, pub Box<dyn Sentence>);

impl Sentence for Implication {
	fn symbols(&self) -> Vec<SymbolName> {
		let mut symbols = self.0.symbols();

		for symbol in self.1.symbols() {
			if !symbols.contains(&symbol) {
				symbols.push(symbol)
			}
		}

		symbols
	}

	fn evaluate(&self, model: &Model) -> bool {
		!self.0.evaluate(model) || self.1.evaluate(model)
	}
}

#[macro_export]
macro_rules! implication {
	($antecedent:expr, $consequent:expr) => {
		$crate::connectives::implication::Implication(Box::new($antecedent), Box::new($consequent))
	};
}
