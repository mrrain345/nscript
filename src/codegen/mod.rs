use crate::{nscript::{environment::Environment, any_value::AnyValue}, parser::expressions::Expression};

mod arithmetic;
// mod assignment;
// mod bitwise;
// mod comparison;
// mod logical;

mod let_;
mod var;
mod assign;
mod call;
mod literals;

impl Expression {
  pub fn codegen<'ctx>(&self, env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
    match self {
      Expression::Integer(value) => literals::integer(env, *value),
      Expression::Number(value) => literals::number(env, *value),
      Expression::Identifier(name) => literals::identifier(env, name),

      Expression::Add(left, right) => arithmetic::add(env, left, right),
      Expression::Sub(left, right) => arithmetic::sub(env, left, right),
      Expression::Mul(left, right) => arithmetic::mul(env, left, right),
      Expression::Div(left, right) => arithmetic::div(env, left, right),
      Expression::Modulo(left, right) => arithmetic::modulo(env, left, right),
      Expression::Power(left, right) => arithmetic::power(env, left, right),
      // Expression::Minus(expr) => arithmetic::minus(env, expr),
      // Expression::Plus(expr) => arithmetic::plus(env, expr),

      Expression::Let { name, type_, value } => let_::let_(env, name, type_, value),
      Expression::Var { name, type_, value } => var::var(env, name, type_, value),
      Expression::Assign { name, value } => assign::assign(env, name, value),

      Expression::Call { name, args } => call::call(env, name, args),

      _ => panic!("Parser error: unimplmented expression `{:?}`", self),
    }
  }
}