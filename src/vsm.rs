use crate::code::{Code, OperationCode, Instruction};
pub struct Vsm {
    code : Code,
    program_counter: i32,
    stack_pointer: i32,
    max_stack_pointer: i32,
    global_top_address: i32,
    frame_top_address: i32,
}
