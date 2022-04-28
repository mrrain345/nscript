use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn call<'ctx>(env: &mut Environment<'ctx>, name: &String, args: &[Expression]) -> AnyValue<'ctx> {
  let mut fn_args = Vec::new();
  for arg in args {
    let arg = arg.codegen(env);
    if arg.is_none() {
      panic!("Parser error: invalid expression")
    } else {
      let arg = arg.unwrap();

      if arg.is_int_value() {
        fn_args.push(arg.into_int_value().into())

      } else if arg.is_float_value() {
        fn_args.push(arg.into_float_value().into())

      } else { panic!("Parser error: invalid argument type") }
    }
  }

  if let Some(function) = env.state.get_function(&name) {
    let res = env.builder.build_call(function.clone(), fn_args.as_slice(), &name);
    let res = res.try_as_basic_value().left();
    match res {
      Some(res) => {
        Some(res.into()).into()
      },
      None => None.into(),
    }
  }
  else {
    panic!("Parser error: function not found");
  }
}