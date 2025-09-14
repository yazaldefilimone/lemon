pub mod context;
pub mod synthesis;
pub mod types;

pub mod expressions;
pub mod statements;
pub mod variables;

pub fn check(ctx: &mut context::Context, module: &ast::Module) {}
