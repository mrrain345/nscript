use inkwell::IntPredicate;

use super::environment::Environment;

pub fn fn_print<'ctx>(env: &mut Environment<'ctx>) -> Option<()> {
  let name = "print";

  // Types
  let i32_type = env.context.i32_type();
  let void_type = env.context.void_type();

  // Declare the putchar function (fn putchar(char: i32) -> null)
  let putchar_type = void_type.fn_type(&[i32_type.into()], false);
  env.module.add_function("putchar", putchar_type, None);

  // Create the print function (fn print(digit: i32) -> null)
  let fn_type = void_type.fn_type(&[i32_type.into()], false);
  let function = env.module.add_function(name, fn_type, None);
  
  // Create blocks
  let entry_block = env.context.append_basic_block(function, "entry");
  let if0_block = env.context.append_basic_block(function, "if0");
  let loop0_block = env.context.append_basic_block(function, "loop0");
  let loop1_block = env.context.append_basic_block(function, "loop1");
  let exit_block = env.context.append_basic_block(function, "exit");

  // Constants
  let const_0 = i32_type.const_int(0, false);
  let const_1 = i32_type.const_int(1, false);
  let const_10 = i32_type.const_int(10, false);
  let const_11 = i32_type.const_int(11, false);
  let newline = i32_type.const_int(10, false);
  let offset = i32_type.const_int(b'0' as u64, false);
  let minus = i32_type.const_int(b'-' as u64, false);

  // Get the putchar function
  let putchar = env.module.get_function("putchar")?;
  
  // --- Entry block ---
  env.builder.position_at_end(entry_block);

  // Create local variables
  let number = env.builder.build_alloca(i32_type, name);
  let iter = env.builder.build_alloca(i32_type, name);

  // Create an array of 11 elements
  let array = env.builder.build_array_alloca(i32_type, const_11, name);

  // Get an argument and store it
  let arg = function.get_nth_param(0)?.into_int_value();
  env.builder.build_store(number, arg);

  // Clear the iterator
  env.builder.build_store(iter, const_0);

  // Check if number is negative
  let num = env.builder.build_load(number, name).into_int_value();
  let is_negative = env.builder.build_int_compare(IntPredicate::SLT, num, const_0, name);

  // If number is negative, print a minus sign
  env.builder.build_conditional_branch(is_negative, if0_block, loop0_block);


  // --- If block ---
  env.builder.position_at_end(if0_block);

  // Print a minus sign
  env.builder.build_call(putchar, &[minus.into()], "putchar");

  // Calculate the absolute value of the number
  let num = env.builder.build_load(number, name).into_int_value();
  let num = env.builder.build_int_sub(const_0, num, name);
  env.builder.build_store(number, num);

  // Jump to the loop0 block
  env.builder.build_unconditional_branch(loop0_block);


  // --- Loop0 block ---
  env.builder.position_at_end(loop0_block);

  // Convert a number to an ASCII character
  let num = env.builder.build_load(number, name).into_int_value();
  let digit = env.builder.build_int_signed_rem(num, const_10, name);
  let ascii = env.builder.build_int_add(digit, offset, name);

  // Store a digit in the array
  let index = env.builder.build_load(iter, name).into_int_value();
  let ptr = unsafe { env.builder.build_gep(array, &[index], name) };
  env.builder.build_store(ptr, ascii);

  // Divide the number by 10 and store it
  let num = env.builder.build_int_signed_div(num, const_10, name);
  env.builder.build_store(number, num);

  // Increment the iterator
  let index = env.builder.build_load(iter, name).into_int_value();
  let new_iter = env.builder.build_int_add(index, const_1, name);
  env.builder.build_store(iter, new_iter);

  // If the number is zero, jump to the loop1 block, else jump to on the beginning of the loop0
  let num = env.builder.build_load(number, name).into_int_value();
  let is_zero = env.builder.build_int_compare(IntPredicate::EQ, num, const_0, name);
  env.builder.build_conditional_branch(is_zero, loop1_block, loop0_block);


  // --- Loop1 block ---
  env.builder.position_at_end(loop1_block);

  // Decrement the iterator
  let index = env.builder.build_load(iter, name).into_int_value();
  let new_iter = env.builder.build_int_sub(index, const_1, name);
  env.builder.build_store(iter, new_iter);

  // Print a character from the array
  let index = env.builder.build_load(iter, name).into_int_value();
  let ptr = unsafe { env.builder.build_gep(array, &[index], name) };
  let ascii = env.builder.build_load(ptr, name).into_int_value();
  env.builder.build_call(putchar, &[ascii.into()], name);

  // If the iterator is zero, jump to the exit block, else jump to on the beginning of the loop1
  let index = env.builder.build_load(iter, name).into_int_value();
  let cmp = env.builder.build_int_compare(IntPredicate::EQ, index, const_0, name);
  env.builder.build_conditional_branch(cmp, exit_block, loop1_block);

  // --- Exit block ---
  env.builder.position_at_end(exit_block);

  // Print a newline
  env.builder.build_call(putchar, &[newline.into()], name);

  // Return
  env.builder.build_return(None);
  Some(())
}