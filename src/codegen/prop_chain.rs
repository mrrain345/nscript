use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn prop_chain<'ctx>(env: &mut Environment<'ctx>, object: &Expression, chain: &[String]) -> AnyValue<'ctx> {
  // Get the object
  let object = object.codegen(env);
  
  // Get the property
  let property = chain.iter().fold(object, |object, property| {
    match object {
      AnyValue::Ptr{ ptr, type_ } => {
        if !type_.is_object() {
          panic!("Invalid type");
        }

        let class = type_.into_object();

        let positon = class.position(property)
          .expect(format!("Property `{property}` not found").as_str());
        
        let type_ = class.get_property(positon).type_;
        
        let struct_value = env.builder.build_load(ptr, class.name_or_default()).into_struct_value();
        let val = env.builder.build_extract_value(struct_value, positon as u32, &property).unwrap();

        if val.is_struct_value() {
          // AnyValue::Object {
          //   struct_value: val.into_struct_value(),
          //   class: type_.into_object()
          // }
          todo!()
        } else {
          match val {
            val if val.is_int_value() && val.into_int_value().get_type().get_bit_width() == 1 => AnyValue::Boolean(val.into_int_value()),
            val if val.is_int_value() => AnyValue::Integer(val.into_int_value()),
            val if val.is_float_value() => AnyValue::Number(val.into_float_value()),
            _ => panic!("Parser error: invalid type of property `{property}`"),
          }
        }
      },
      _ => panic!("Parser error: invalid type of object"),
    }
  });

  property
}