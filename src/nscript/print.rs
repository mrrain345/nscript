use super::NScript;
use inkwell::IntPredicate;

impl<'ctx> NScript<'ctx> {
  pub fn fn_print(&self) -> Option<()> {
    let name = "print";

    // Types
    let i32_type = self.context.i32_type();
    let void_type = self.context.void_type();

    // Declare the putchar function (fn putchar(char: i32) -> null)
    let putchar_type = void_type.fn_type(&[i32_type.into()], false);
    self.module.add_function("putchar", putchar_type, None);

    // Create the print function (fn print(digit: i32) -> null)
    let fn_type = void_type.fn_type(&[i32_type.into()], false);
    let function = self.module.add_function(name, fn_type, None);
    
    // Create blocks
    let entry_block = self.context.append_basic_block(function, "entry");
    let if0_block = self.context.append_basic_block(function, "if0");
    let loop0_block = self.context.append_basic_block(function, "loop0");
    let loop1_block = self.context.append_basic_block(function, "loop1");
    let exit_block = self.context.append_basic_block(function, "exit");

    // Constants
    let const_0 = i32_type.const_int(0, false);
    let const_1 = i32_type.const_int(1, false);
    let const_10 = i32_type.const_int(10, false);
    let const_11 = i32_type.const_int(11, false);
    let newline = i32_type.const_int(10, false);
    let offset = i32_type.const_int(b'0' as u64, false);
    let minus = i32_type.const_int(b'-' as u64, false);

    // Get the putchar function
    let putchar = self.module.get_function("putchar")?;
    
    // --- Entry block ---
    self.builder.position_at_end(entry_block);

    // Create local variables
    let number = self.builder.build_alloca(i32_type, name);
    let iter = self.builder.build_alloca(i32_type, name);

    // Create an array of 11 elements
    let array = self.builder.build_array_alloca(i32_type, const_11, name);

    // Get an argument and store it
    let arg = function.get_nth_param(0)?.into_int_value();
    self.builder.build_store(number, arg);

    // Clear the iterator
    self.builder.build_store(iter, const_0);

    // Check if number is negative
    let num = self.builder.build_load(number, name).into_int_value();
    let is_negative = self.builder.build_int_compare(IntPredicate::SLT, num, const_0, name);

    // If number is negative, print a minus sign
    self.builder.build_conditional_branch(is_negative, if0_block, loop0_block);


    // --- If block ---
    self.builder.position_at_end(if0_block);

    // Print a minus sign
    self.builder.build_call(putchar, &[minus.into()], "putchar");

    // Calculate the absolute value of the number
    let num = self.builder.build_load(number, name).into_int_value();
    let num = self.builder.build_int_sub(const_0, num, name);
    self.builder.build_store(number, num);

    // Jump to the loop0 block
    self.builder.build_unconditional_branch(loop0_block);


    // --- Loop0 block ---
    self.builder.position_at_end(loop0_block);

    // Convert a number to an ASCII character
    let num = self.builder.build_load(number, name).into_int_value();
    let digit = self.builder.build_int_signed_rem(num, const_10, name);
    let ascii = self.builder.build_int_add(digit, offset, name);

    // Store a digit in the array
    let index = self.builder.build_load(iter, name).into_int_value();
    let ptr = unsafe { self.builder.build_gep(array, &[index], name) };
    self.builder.build_store(ptr, ascii);

    // Divide the number by 10 and store it
    let num = self.builder.build_int_signed_div(num, const_10, name);
    self.builder.build_store(number, num);

    // Increment the iterator
    let index = self.builder.build_load(iter, name).into_int_value();
    let new_iter = self.builder.build_int_add(index, const_1, name);
    self.builder.build_store(iter, new_iter);

    // If the number is zero, jump to the loop1 block, else jump to on the beginning of the loop0
    let num = self.builder.build_load(number, name).into_int_value();
    let is_zero = self.builder.build_int_compare(IntPredicate::EQ, num, const_0, name);
    self.builder.build_conditional_branch(is_zero, loop1_block, loop0_block);


    // --- Loop1 block ---
    self.builder.position_at_end(loop1_block);

    // Decrement the iterator
    let index = self.builder.build_load(iter, name).into_int_value();
    let new_iter = self.builder.build_int_sub(index, const_1, name);
    self.builder.build_store(iter, new_iter);

    // Print a character from the array
    let index = self.builder.build_load(iter, name).into_int_value();
    let ptr = unsafe { self.builder.build_gep(array, &[index], name) };
    let ascii = self.builder.build_load(ptr, name).into_int_value();
    self.builder.build_call(putchar, &[ascii.into()], name);

    // If the iterator is zero, jump to the exit block, else jump to on the beginning of the loop1
    let index = self.builder.build_load(iter, name).into_int_value();
    let cmp = self.builder.build_int_compare(IntPredicate::EQ, index, const_0, name);
    self.builder.build_conditional_branch(cmp, exit_block, loop1_block);

    // --- Exit block ---
    self.builder.position_at_end(exit_block);

    // Print a newline
    self.builder.build_call(putchar, &[newline.into()], name);

    self.builder.build_return(None);
    return Some(());
  }
}