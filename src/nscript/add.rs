use super::NScript;

impl<'ctx> NScript<'ctx> {
  pub fn fn_add(&self) -> Option<()> {
    let name = "add";

    // Types
    let i64_type = self.context.i64_type();

    // Declare the add function
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
    let function = self.module.add_function(name, fn_type, None);

    // Create an entry block
    let basic_block = self.context.append_basic_block(function, "entry");
    self.builder.position_at_end(basic_block);

    // Get the arguments
    let x = function.get_nth_param(0)?.into_int_value();
    let y = function.get_nth_param(1)?.into_int_value();

    // Add the arguments
    let add = self.builder.build_int_add(x, y, name);

    self.builder.build_return(Some(&add));
    return Some(());
  }
}