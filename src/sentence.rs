use crate::model::Model;
use crate::symbol::SymbolName;

pub trait Sentence {
	fn symbols(&self) -> Vec<SymbolName>;
	fn evaluate(&self, model: &Model) -> bool;
}
