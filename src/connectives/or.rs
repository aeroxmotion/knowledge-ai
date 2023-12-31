use crate::{merge_symbols, model::Model, sentence::Sentence, symbol::SymbolName};

pub struct Or(pub Vec<Box<dyn Sentence>>);

impl Sentence for Or {
	fn symbols(&self) -> Vec<SymbolName> {
		merge_symbols!(&self.0[..])
	}

	fn evaluate(&self, model: &Model) -> bool {
		for sentence in &self.0 {
			if sentence.evaluate(model) {
				return true;
			}
		}

		false
	}
}

impl Or {
	pub fn add(&mut self, sentence: Box<dyn Sentence>) {
		self.0.push(sentence)
	}
}

#[macro_export]
macro_rules! or {
	($($x:expr),+ $(,)?) => ($crate::connectives::or::Or(vec![$(Box::new($x)),*]));
}
