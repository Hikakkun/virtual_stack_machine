use std::io;
use virtual_stack_machine::code::*;

fn main() -> io::Result<()> {
    let mut code = Code::new();

    code.read("/home/hikaru/prog/mini_c/virtual_stack_machine/tests/full.vsm")?;

    code.print();

    Ok(())
}
