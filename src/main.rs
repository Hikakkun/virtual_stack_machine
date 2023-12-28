use std::io;
use virtual_stack_machine::vsm::{*, self};
use std::env;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <vsm_file>", &args[0]);
        std::process::exit(1);
    }

    let vsm_file = &args[1];
    let mut vsm = Vsm::new();

    vsm.read_code(vsm_file).unwrap();

    vsm.exec_code().unwrap();
    Ok(())
}
