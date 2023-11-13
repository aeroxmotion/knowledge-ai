use crate::{model::Model, sentence::Sentence};

pub type SymbolName = String;

#[derive(Clone, Copy)]
pub struct Symbol<T: Into<SymbolName> + Copy>(pub T);

impl<T: Into<SymbolName> + Copy> Sentence for Symbol<T> {
	fn symbols(&self) -> Vec<SymbolName> {
		vec![self.0.into()]
	}

	fn evaluate(&self, model: &Model) -> bool {
		match model.get(&self.0.into()) {
			Some(name) => *name,
			None => false,
		}
	}
}

/// To create boxed symbols...
#[macro_export]
macro_rules! symbol {
	($symbol:expr) => {
		Box::new($symbol)
	};
}
