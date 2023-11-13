use crate::{merge_symbols, model::Model, sentence::Sentence, symbol::SymbolName};

pub struct Biconditional(pub Box<dyn Sentence>, pub Box<dyn Sentence>);

impl Sentence for Biconditional {
	fn symbols(&self) -> Vec<SymbolName> {
		merge_symbols!(&self.0, &self.1)
	}

	fn evaluate(&self, model: &Model) -> bool {
		self.0.evaluate(model) == self.1.evaluate(model)
	}
}

#[macro_export]
macro_rules! biconditional {
	($left:expr, $right:expr) => {
		$crate::connectives::biconditional::Biconditional(
			Box::new($antecedent),
			Box::new($consequent),
		)
	};
}
