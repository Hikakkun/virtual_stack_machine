use std::io;
use virtual_stack_machine::vsm::*;
fn main() -> io::Result<()> {

    let mut vsm = Vsm::new();

    vsm.read_code("/home/hikaru/prog/mini_c/virtual_stack_machine/V_1.vsm")?;

    vsm.print_code();

    Ok(())
}