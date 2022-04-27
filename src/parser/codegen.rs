use inkwell::values::BasicMetadataValueEnum;

use crate::nscript::{environment::Environment, any_value::AnyValue};

use super::expressions::*;

impl Expression {
  pub fn codegen<'ctx>(&self, env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
    match self {
      Expression::Integer(value) => {
        env.context.i32_type().const_int(*value as u64, false).into()
      }
      
      Expression::Number(value) => {
        env.context.f64_type().const_float(*value).into()
      }

      Expression::Add(left, right) => {
        let left = left.codegen(env);
        let right = right.codegen(env);

        if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
          if left.is_int_value() && right.is_int_value() {
            // Integer + Integer
            let left = left.into_int_value();
            let right = right.into_int_value();
            let result = env.builder.build_int_add(left, right, "add");
            return result.into();
          }
          else if left.is_float_value() && right.is_float_value() {
            // Float + Float
            let left = left.into_float_value();
            let right = right.into_float_value();
            let result = env.builder.build_float_add(left, right, "add");
            return result.into();
          }
          else if left.is_int_value() && right.is_float_value() {
            // Integer + Float
            let left = left.into_int_value();
            let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
            let right = right.into_float_value();
            let result = env.builder.build_float_add(left, right, "add");
            return result.into();
          }
          else if left.is_float_value() && right.is_int_value() {
            // Float + Integer
            let right = right.into_int_value();
            let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
            let left = left.into_float_value();
            let result = env.builder.build_float_add(left, right, "add");
            return result.into();
          }
          else { panic!("Parser error: Addition of incompatible types") }
        }
        else { panic!("Parser error: invalid expression") }
      },

      Expression::Sub(left, right) => {
        let left = left.codegen(env);
        let right = right.codegen(env);

        if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
          if left.is_int_value() && right.is_int_value() {
            // Integer - Integer
            let left = left.into_int_value();
            let right = right.into_int_value();
            let result = env.builder.build_int_sub(left, right, "sub");
            return result.into();
          }
          else if left.is_float_value() && right.is_float_value() {
            // Float - Float
            let left = left.into_float_value();
            let right = right.into_float_value();
            let result = env.builder.build_float_sub(left, right, "sub");
            return result.into();
          }
          else if left.is_int_value() && right.is_float_value() {
            // Integer - Float
            let left = left.into_int_value();
            let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
            let right = right.into_float_value();
            let result = env.builder.build_float_sub(left, right, "sub");
            return result.into();
          }
          else if left.is_float_value() && right.is_int_value() {
            // Float - Integer
            let right = right.into_int_value();
            let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
            let left = left.into_float_value();
            let result = env.builder.build_float_sub(left, right, "sub");
            return result.into();
          }
          else { panic!("Parser error: Subtraction of incompatible types") }
        }
        else { panic!("Parser error: invalid expression") }
      },

      Expression::Mul(left, right) => {
        let left = left.codegen(env);
        let right = right.codegen(env);

        if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
          if left.is_int_value() && right.is_int_value() {
            // Integer * Integer
            let left = left.into_int_value();
            let right = right.into_int_value();
            let result = env.builder.build_int_mul(left, right, "mul");
            return result.into();
          }
          else if left.is_float_value() && right.is_float_value() {
            // Float * Float
            let left = left.into_float_value();
            let right = right.into_float_value();
            let result = env.builder.build_float_mul(left, right, "mul");
            return result.into();
          }
          else if left.is_int_value() && right.is_float_value() {
            // Integer * Float
            let left = left.into_int_value();
            let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
            let right = right.into_float_value();
            let result = env.builder.build_float_mul(left, right, "mul");
            return result.into();
          }
          else if left.is_float_value() && right.is_int_value() {
            // Float * Integer
            let right = right.into_int_value();
            let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
            let left = left.into_float_value();
            let result = env.builder.build_float_mul(left, right, "mul");
            return result.into();
          }
          else { panic!("Parser error: Multiplication of incompatible types") }
        }
        else { panic!("Parser error: invalid expression") }
      },

      Expression::Div(left, right) => {
        let left = left.codegen(env);
        let right = right.codegen(env);

        if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
          if left.is_int_value() && right.is_int_value() {
            // Integer / Integer
            let left = left.into_int_value();
            let right = right.into_int_value();
            let result = env.builder.build_int_signed_div(left, right, "div");
            return result.into();
          }
          else if left.is_float_value() && right.is_float_value() {
            // Float / Float
            let left = left.into_float_value();
            let right = right.into_float_value();
            let result = env.builder.build_float_div(left, right, "div");
            return result.into();
          }
          else if left.is_int_value() && right.is_float_value() {
            // Integer / Float
            let left = left.into_int_value();
            let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
            let right = right.into_float_value();
            let result = env.builder.build_float_div(left, right, "div");
            return result.into();
          }
          else if left.is_float_value() && right.is_int_value() {
            // Float / Integer
            let right = right.into_int_value();
            let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
            let left = left.into_float_value();
            let result = env.builder.build_float_div(left, right, "div");
            return result.into();
          }
          else { panic!("Parser error: Division of incompatible types") }
        }
        else { panic!("Parser error: invalid expression") }
      },

      Expression::Mod(left, right) => {
        let left = left.codegen(env);
        let right = right.codegen(env);

        if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
          if left.is_int_value() && right.is_int_value() {
            // Integer % Integer
            let left = left.into_int_value();
            let right = right.into_int_value();
            let result = env.builder.build_int_signed_rem(left, right, "mod");
            return result.into();
          }
          else { panic!("Parser error: Modulo of incompatible types") }
        }
        else { panic!("Parser error: invalid expression") }
      },

      // Expression::Pow(left, right) => {
      //   let left = left.codegen(env);
      //   let right = right.codegen(env);

      //   if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
      //     if left.is_int_value() && right.is_int_value() {
      //       // Integer ^ Integer
      //       let left = left.into_int_value();
      //       let right = right.into_int_value();
      //       let result = env.builder.build_int_signed_pow(left, right, "pow");
      //     urn Some(result.into();
      //     }
      //     else { panic!("Parser error: Power of incompatible types") }
      //   }
      //   else { panic!("Parser error: invalid expression") }
      // },

      Expression::Neg(expr) => {
        let expr = expr.codegen(env);

        if let Some(expr) = expr.into_option() {
          if expr.is_int_value() {
            // Integer negation
            let expr = expr.into_int_value();
            let result = env.builder.build_int_neg(expr, "neg");
            return result.into();
          }
          else if expr.is_float_value() {
            // Float negation
            let expr = expr.into_float_value();
            let result = env.builder.build_float_neg(expr, "neg");
            return result.into();
          }
          else { panic!("Parser error: Negation of incompatible types") }
        }
        else { panic!("Parser error: invalid expression") }
      },



      Expression::Let { name, type_, value } => {
        let value = value.codegen(env);

        if value.is_none() {
          panic!("Parser error: invalid expression");
        }

        let value = value.unwrap();
        // TODO: Check if type is compatible with value
        let value = env.state.add_label(name.into(), value).into_option()
          .expect(format!("Label `{}` already exists", name).as_str());

        Some(value).into()
      }

      Expression::Var { name, type_, value } => {
        // if value.is_none() {
        //   if type_.is_none() {
        //     panic!("Parser error: you must specify a type or a value for variable `{}`", name);
        //   }
        //   let value = match type_.as_ref().unwrap().0.as_str() {
        //     "Integer" => env.integer(0).into(),
        //     "Number" => env.number(0.0).into(),
        //     "Boolean" => env.boolean(false).into(),
        //     _ => panic!("Parser error: invalid type `{}`", type_.as_ref().unwrap().0)
        //   };
        //   let value = env.state.add_variable(name.into(), value).into_option()
        //     .expect(format!("Variable `{}` already exists", name).as_str());

        //   return Some(value).into();
        // }

        let value = value.as_ref().unwrap().codegen(env);

        if value.is_none() {
          panic!("Parser error: invalid expression");
        }

        let value = value.unwrap();
        // TODO: Check if type is compatible with value
        env.state.add_variable(name.into(), value);

        Some(value).into()
      }
          
      Expression::Identifier(name) => {
        if let Some(value) = env.state.get(&name, None).into_option() {
          return Some(value).into();
        }
        else {
          panic!("Parser error: label not found");
        }
      }

      Expression::Call(name, args) => {
        let mut fn_args = Vec::new();
        for arg in args {
          let arg = arg.codegen(env);
          if arg.is_none() {
            panic!("Parser error: invalid expression")
          } else {
            let arg = arg.unwrap();

            if arg.is_int_value() {
              fn_args.push(BasicMetadataValueEnum::IntValue(arg.into_int_value()))

            } else if arg.is_float_value() {
              fn_args.push(BasicMetadataValueEnum::FloatValue(arg.into_float_value()))

            } else { panic!("Parser error: invalid argument type") }
          }
        }

        if let Some(function) = env.state.get_function(&name) {
          let res = env.builder.build_call(function.clone(), fn_args.as_slice(), &name);
          let res = res.try_as_basic_value().left();
          match res {
            Some(res) => {
              return Some(res.into()).into();
            },
            None => None.into(),
          }
        }
        else {
          panic!("Parser error: function not found");
        }
      }
      
      

      _ => panic!("Parser error: unimplmented expression `{:?}`", self),
    }
  }
}