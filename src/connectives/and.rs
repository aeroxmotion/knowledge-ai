use crate::{merge_symbols, model::Model, sentence::Sentence, symbol::SymbolName};

pub struct And(pub Vec<Box<dyn Sentence>>);

impl Sentence for And {
	fn symbols(&self) -> Vec<SymbolName> {
		merge_symbols!(&self.0[..])
	}

	fn evaluate(&self, model: &Model) -> bool {
		for sentence in &self.0 {
			if !sentence.evaluate(model) {
				return false;
			}
		}

		true
	}
}

impl And {
	pub fn add(&mut self, sentence: Box<dyn Sentence>) {
		self.0.push(sentence)
	}
}

#[macro_export]
macro_rules! and {
	($($x:expr),+ $(,)?) => ($crate::connectives::and::And(vec![$(Box::new($x)),*]));
}
