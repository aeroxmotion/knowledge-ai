use crate::{model::Model, sentence::Sentence, symbol::SymbolName};

pub struct Not(pub Box<dyn Sentence>);

impl Sentence for Not {
	fn symbols(&self) -> Vec<SymbolName> {
		self.0.symbols()
	}

	fn evaluate(&self, model: &Model) -> bool {
		!self.0.evaluate(model)
	}
}

#[macro_export]
macro_rules! not {
	($sentence:expr) => {
		$crate::connectives::not::Not(Box::new($sentence))
	};
}
