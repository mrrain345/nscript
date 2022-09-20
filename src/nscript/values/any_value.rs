use std::fmt::{Display, Formatter, Error};

use inkwell::values::{PointerValue, AnyValueEnum, BasicValueEnum};

use crate::nscript::{Environment, AnyType};

use super::{integer::Integer, number::Number, boolean::Boolean, Function, Object, Class, Value, Null, Ref, Type};

#[derive(Debug, Clone)]
pub enum AnyValue<'ctx> {
  Integer(Integer<'ctx>),
  Number(Number<'ctx>),
  Boolean(Boolean<'ctx>),
  Null,
  Function(Function<'ctx>),
  Object(Object<'ctx>),
  Class(Class<'ctx>),
  Ref(Ref<'ctx>),
  Type(Type<'ctx>),
}

impl<'ctx> AnyValue<'ctx> {
  pub fn deref(self, env: &Environment<'ctx>) -> AnyValue<'ctx> {
    match self {
      AnyValue::Ref(ref_) => {
        let value = env.borrow_mut().builder.build_load(ref_.ptr, "deref");
        ref_.type_.create_value(env, value.into())
      },
      _ => self,
    }
  }

  pub fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    match self {
      AnyValue::Integer(value) => value.allocate(env),
      AnyValue::Number(value) => value.allocate(env),
      AnyValue::Boolean(value) => value.allocate(env),
      AnyValue::Null => Null::llvm_null_value(env),
      AnyValue::Object(value) => value.allocate(env),
      AnyValue::Class(value) => value.allocate(env),
      AnyValue::Function(value) => value.allocate(env),
      AnyValue::Ref(value) => value.allocate(env),
      AnyValue::Type(value) => value.allocate(env), 
    }
  }

  pub fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    match self {
      AnyValue::Integer(value) => value.store(env, ptr),
      AnyValue::Number(value) => value.store(env, ptr),
      AnyValue::Boolean(value) => value.store(env, ptr),
      AnyValue::Null => Null.store(env, ptr),
      AnyValue::Object(value) => value.store(env, ptr),
      AnyValue::Class(value) => value.store(env, ptr),
      AnyValue::Function(value) => value.store(env, ptr),
      AnyValue::Ref(value) => value.store(env, ptr),
      AnyValue::Type(value) => value.store(env, ptr),
    };
  }

  pub fn get_type(&self) -> AnyType<'ctx> {
    match self {
      AnyValue::Integer(value) => value.get_type(),
      AnyValue::Number(value) => value.get_type(),
      AnyValue::Boolean(value) => value.get_type(),
      AnyValue::Null => Null.get_type(),
      AnyValue::Function(value) => value.get_type(),
      AnyValue::Object(value) => value.get_type(),
      AnyValue::Class(value) => value.get_type(),
      AnyValue::Ref(value) => value.get_type(),
      AnyValue::Type(value) => value.get_type(),
    }
  }

  pub fn silent_cast(&self, env: &Environment<'ctx>, type_: &AnyType) -> Option<AnyValue<'ctx>> {
    if self.get_type() == *type_ {
      return Some(self.clone());
    }

    match self {
      AnyValue::Integer(value) => value.silent_cast(env, type_),
      AnyValue::Number(value) => value.silent_cast(env, type_),
      AnyValue::Boolean(value) => value.silent_cast(env, type_),
      AnyValue::Null => Null.silent_cast(env, type_),
      AnyValue::Function(value) => value.silent_cast(env, type_),
      AnyValue::Object(value) => value.silent_cast(env, type_),
      AnyValue::Class(value) => value.silent_cast(env, type_),
      AnyValue::Ref(value) => value.silent_cast(env, type_),
      AnyValue::Type(value) => value.silent_cast(env, type_),
    }
  }

  pub fn cast(&self, env: &Environment<'ctx>, type_: &AnyType) -> Option<AnyValue<'ctx>> {
    if self.get_type() == *type_ {
      return Some(self.clone());
    }

    match self {
      AnyValue::Integer(value) => value.cast(env, type_),
      AnyValue::Number(value) => value.cast(env, type_),
      AnyValue::Boolean(value) => value.cast(env, type_),
      AnyValue::Null => Null.cast(env, type_),
      AnyValue::Function(value) => value.cast(env, type_),
      AnyValue::Object(value) => value.cast(env, type_),
      AnyValue::Class(value) => value.cast(env, type_),
      AnyValue::Ref(value) => value.cast(env, type_),
      AnyValue::Type(value) => value.cast(env, type_),
    }
  }

  // pub fn op_add(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
  //   match self {
  //     AnyValue::Integer(value) => value.op_add(env, other),
  //     AnyValue::Number(value) => value.op_add(env, other),
  //     AnyValue::Boolean(value) => value.op_add(env, other),
  //     AnyValue::Null => Null.op_add(env, other),
  //     AnyValue::Function(value) => value.op_add(env, other),
  //     AnyValue::Object(value) => value.op_add(env, other),
  //     AnyValue::Class(value) => value.op_add(env, other),
  //     AnyValue::Ref(value) => value.op_add(env, other),
  //   }
  // }

  pub fn llvm_value(&self, env: &Environment<'ctx>) -> AnyValueEnum<'ctx> {
    match self {
      AnyValue::Integer(value) => value.llvm_value(env).into(),
      AnyValue::Number(value) => value.llvm_value(env).into(),
      AnyValue::Boolean(value) => value.llvm_value(env).into(),
      AnyValue::Null => Null.llvm_value(env).into(),
      AnyValue::Function(value) => value.llvm_value(env).into(),
      AnyValue::Object(value) => value.llvm_value(env).into(),
      AnyValue::Class(value) => value.llvm_value(env).into(),
      AnyValue::Ref(value) => value.llvm_value(env).into(),
      AnyValue::Type(value) => value.llvm_value(env).into(),
    }
  }

  pub fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    match self {
      AnyValue::Integer(value) => value.llvm_basic_value(env),
      AnyValue::Number(value) => value.llvm_basic_value(env),
      AnyValue::Boolean(value) => value.llvm_basic_value(env),
      AnyValue::Null => Null.llvm_basic_value(env),
      AnyValue::Function(value) => value.llvm_basic_value(env),
      AnyValue::Object(value) => value.llvm_basic_value(env),
      AnyValue::Class(value) => value.llvm_basic_value(env),
      AnyValue::Ref(value) => value.llvm_basic_value(env),
      AnyValue::Type(value) => value.llvm_basic_value(env),
    }
  }


  pub fn is_integer(&self) -> bool {
    matches!(self, AnyValue::Integer(_))
  }

  pub fn is_number(&self) -> bool {
    matches!(self, AnyValue::Number(_))
  }

  pub fn is_boolean(&self) -> bool {
    matches!(self, AnyValue::Boolean(_))
  }

  pub fn is_null(&self) -> bool {
    matches!(self, AnyValue::Null)
  }

  pub fn is_function(&self) -> bool {
    matches!(self, AnyValue::Function(_))
  }

  pub fn is_object(&self) -> bool {
    matches!(self, AnyValue::Object(_))
  }

  pub fn is_class(&self) -> bool {
    matches!(self, AnyValue::Class(_))
  }

  pub fn is_ref(&self) -> bool {
    matches!(self, AnyValue::Ref(_))
  }

  pub fn is_type(&self) -> bool {
    matches!(self, AnyValue::Type(_))
  }


  pub fn into_integer(self) -> Option<Integer<'ctx>> {
    if let AnyValue::Integer(value) = self { Some(value) } else { None }
  }

  pub fn into_number(self) -> Option<Number<'ctx>> {
    if let AnyValue::Number(value) = self { Some(value) } else { None }
  }

  pub fn into_boolean(self) -> Option<Boolean<'ctx>> {
    if let AnyValue::Boolean(value) = self { Some(value) } else { None }
  }

  pub fn into_null(self) -> Option<Null> {
    if let AnyValue::Null = self { Some(Null) } else { None }
  }

  pub fn into_function(self) -> Option<Function<'ctx>> {
    if let AnyValue::Function(value) = self { Some(value) } else { None }
  }

  pub fn into_object(self) -> Option<Object<'ctx>> {
    if let AnyValue::Object(value) = self { Some(value) } else { None }
  }

  pub fn into_class(self) -> Option<Class<'ctx>> {
    if let AnyValue::Class(value) = self { Some(value) } else { None }
  }

  pub fn into_ref(self) -> Option<Ref<'ctx>> {
    if let AnyValue::Ref(value) = self { Some(value) } else { None }
  }

  pub fn into_type(self) -> Option<Type<'ctx>> {
    if let AnyValue::Type(value) = self { Some(value) } else { None }
  }
}

impl<'ctx> Display for AnyValue<'ctx> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      AnyValue::Integer(_) => write!(f, "Integer"),
      AnyValue::Number(_) => write!(f, "Number"),
      AnyValue::Boolean(_) => write!(f, "Boolean"),
      AnyValue::Null => write!(f, "null"),
      AnyValue::Function(function) => write!(f, "fn {}({:#?}) -> {:?}", function.name().unwrap_or_default(), function.args(), function.return_type()),
      AnyValue::Class(class) => write!(f, "Class({})", class.name_or_default()),
      AnyValue::Object(object) => write!(f, "Object({})", object.class().name_or_default()),
      AnyValue::Ref(ref_) => write!(f, "ref {:?}", ref_.type_),
      AnyValue::Type(type_) => write!(f, "type {:?}", type_),
    }
  }
}