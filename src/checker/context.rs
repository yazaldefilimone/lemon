use super::store::FunctionStore;
use super::store::TypeStore;
use crate::checker::types::TypeId;
use crate::cli::CompilerOptions;
use crate::messages::Messages;
use crate::symbols::scope::SymbolsScope;

#[derive(Debug)]
pub struct Context<'a, 'b> {
	pub file_index: u32,

	pub compiler_options: &'b CompilerOptions,

	pub scope_index: u32,
	pub messages: &'b mut Messages<'a>,
	pub return_type: Option<TypeId>,

	pub type_store: &'b mut TypeStore<'a>,
	pub function_store: &'b FunctionStore<'a>,

	pub symbols_scope: SymbolsScope<'a, 'b>,
}
