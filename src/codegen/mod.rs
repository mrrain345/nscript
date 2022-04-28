use crate::{nscript::{environment::Environment, any_value::AnyValue}, parser::expressions::Expression};

mod arithmetic;
mod assignment;
mod bitwise;
mod comparison;
mod logical;
mod statement;

mod call;
mod literals;

impl Expression {
  pub fn codegen<'ctx>(&self, env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
    match self {
      Expression::Integer(value) => literals::integer(env, *value),
      Expression::Number(value) => literals::number(env, *value),
      Expression::String(value) => literals::string(env, value),
      Expression::Boolean(value) => literals::boolean(env, *value),
      Expression::Null => literals::null(env),
      Expression::Identifier(name) => literals::identifier(env, name),

      Expression::Add(left, right) => arithmetic::add(env, left, right),
      Expression::Sub(left, right) => arithmetic::sub(env, left, right),
      Expression::Mul(left, right) => arithmetic::mul(env, left, right),
      Expression::Div(left, right) => arithmetic::div(env, left, right),
      Expression::Modulo(left, right) => arithmetic::modulo(env, left, right),
      Expression::Power(left, right) => arithmetic::power(env, left, right),
      Expression::Minus(value) => arithmetic::minus(env, value),
      Expression::Plus(value) => arithmetic::plus(env, value),

      Expression::And(left, right) => logical::and(env, left, right),
      Expression::Or(left, right) => logical::or(env, left, right),
      Expression::Not(value) => logical::not(env, value),

      Expression::Equal(left, right) => comparison::equal(env, left, right),
      Expression::NotEqual(left, right) => comparison::not_equal(env, left, right),
      Expression::LessThan(left, right) => comparison::less_than(env, left, right),
      Expression::GreaterThan(left, right) => comparison::greater_than(env, left, right),
      Expression::LessOrEqual(left, right) => comparison::less_or_equal(env, left, right),
      Expression::GreaterOrEqual(left, right) => comparison::greater_or_equal(env, left, right),

      Expression::Let { name, type_, value } => statement::let_(env, name, type_, value),
      Expression::Var { name, type_, value } => statement::var(env, name, type_, value),
      Expression::Assign { name, value } => assignment::assign(env, name, value),
      Expression::If { condition, then, else_ } => statement::if_(env, condition, then, else_),

      Expression::Call { name, args } => call::call(env, name, args),

      _ => panic!("Parser error: unimplmented expression `{self:?}`"),
    }
  }
}