use crate::{parser::Expression, nscript::{{AnyValue}, Environment}};

pub fn assign<'ctx>(env: &mut Environment<'ctx>, ptr: &Expression, value: &Expression) -> AnyValue<'ctx> {
  let ptr = ptr.codegen(env);
  let value = value.codegen(env);

  if !ptr.is_ptr() {
    panic!("Invalid pointer `{ptr:?}`");
  }

  let (ptr, type_) = ptr.into_ptr();

  if type_ != value.get_type() {
    panic!("Type mismatch: cannot assign `{value:?}` to `{ptr:?}`", value = value, ptr = ptr);
  }

  match value {
    AnyValue::Integer(value) => {
      env.builder.build_store(ptr, value);
    },
    AnyValue::Number(value) => {
      env.builder.build_store(ptr, value);
    },
    AnyValue::Boolean(value) => {
      env.builder.build_store(ptr, value);
    },
    _ => panic!("Parser error: invalid type `{value:?}`")
  }

  value

  // env.set_variable(name.into(), ptr, type_)
  //   .expect(format!("Variable `{name}` doesn't exist").as_str())
}