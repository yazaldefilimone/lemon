use crate::{checker::types::TypeId, symbols::Symbol};

#[derive(Debug)]
pub struct ScopeStore<'a, 'b> {
	pub scopes: &'b mut Vec<Scope<'a>>,
}

impl<'a, 'b> ScopeStore<'a, 'b> {
	pub fn push_function_scope(
		&mut self,
		return_type: TypeId,
		params: Vec<Symbol<'a>>,
		is_async: bool,
	) {
		let function_scope_state = FunctionScopeState {
			return_type,
			all_paths_return: false,
			params,
			is_async,
			has_explicit_return: false,
		};
		let state = ScopeState::Function(function_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_closure_scope(&mut self, captures: Vec<Symbol<'a>>, return_type: Option<TypeId>) {
		let closure_scope_state = ClosureScopeState { captures, return_type };
		let state = ScopeState::Closure(closure_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_match_arm_scope(&mut self, pattern_symbols: Vec<Symbol<'a>>) {
		let match_arm_scope_state =
			MatchArmScopeState { pattern_symbols, exhaustiveness_checked: false };
		let state = ScopeState::MatchArm(match_arm_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_loop_scope(
		&mut self,
		break_label: Option<&'a str>,
		has_break: bool,
		has_continue: bool,
	) {
		let loop_scope_state = LoopScopeState { break_label, has_break, has_continue };
		let state = ScopeState::Loop(loop_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_block_scope(&mut self, is_const: bool) {
		let block_scope_state = BlockScopeState { is_const };
		let state = ScopeState::Block(block_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_module_scope(&mut self, imports: Vec<Symbol<'a>>) {
		let module_scope_state = ModuleScopeState { imports };
		let state = ScopeState::Module(module_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_global_scope(&mut self, modules: Vec<&'a str>) {
		let global_scope_state = GlobalScopeState { modules };
		let state = ScopeState::Global(global_scope_state);
		self.push_scope(Vec::new(), state);
	}

	pub fn push_scope(&mut self, symbols: Vec<Symbol<'a>>, state: ScopeState<'a>) {
		let parent = self.scopes.last().map(|scope| scope.symbols.len());
		self.scopes.push(Scope { symbols, state, parent });
	}

	pub fn push_scope_with_parent(
		&mut self,
		symbols: Vec<Symbol<'a>>,
		state: ScopeState<'a>,
		parent: usize,
	) {
		self.scopes.push(Scope { symbols, state, parent: Some(parent) });
	}

	pub fn pop_scope(&mut self) -> Option<Scope<'a>> {
		self.scopes.pop()
	}

	pub fn lookup_scope(&mut self, position: usize) -> Option<&mut Scope<'a>> {
		self.scopes.get_mut(position)
	}
}

#[derive(Debug)]
pub struct Scope<'a> {
	pub symbols: Vec<Symbol<'a>>,
	pub state: ScopeState<'a>,
	pub parent: Option<usize>,
}

impl<'a> Scope<'a> {
	pub fn lookup_symbol_by_name(&mut self, name: &str, mark_used: bool) -> Option<&mut Symbol<'a>> {
		self.symbols.iter_mut().find(|symbol| symbol.name == name)
	}
}

#[derive(Debug)]
pub enum ScopeState<'a> {
	Global(GlobalScopeState<'a>),

	Module(ModuleScopeState<'a>),

	Function(FunctionScopeState<'a>),

	Loop(LoopScopeState<'a>),

	Block(BlockScopeState),

	Closure(ClosureScopeState<'a>),

	MatchArm(MatchArmScopeState<'a>),
}

#[derive(Debug)]
pub struct FunctionScopeState<'a> {
	pub return_type: TypeId,
	pub all_paths_return: bool,
	pub params: Vec<Symbol<'a>>,
	pub is_async: bool,
	pub has_explicit_return: bool,
}

#[derive(Debug)]
pub struct ClosureScopeState<'a> {
	pub captures: Vec<Symbol<'a>>,
	return_type: Option<TypeId>,
}

#[derive(Debug)]
pub struct MatchArmScopeState<'a> {
	pub pattern_symbols: Vec<Symbol<'a>>,
	exhaustiveness_checked: bool,
}

#[derive(Debug)]
pub struct LoopScopeState<'a> {
	pub break_label: Option<&'a str>,
	has_break: bool,
	has_continue: bool,
}

#[derive(Debug)]
pub struct BlockScopeState {
	is_const: bool,
}

#[derive(Debug)]
pub struct ModuleScopeState<'a> {
	imports: Vec<Symbol<'a>>,
}
#[derive(Debug)]
pub struct GlobalScopeState<'a> {
	modules: Vec<&'a str>,
}
