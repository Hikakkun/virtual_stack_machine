use std::io;
use virtual_stack_machine::vsm::*;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        eprintln!("Usage: {} <vsm_file> <option>", &args[0]);
        std::process::exit(1);
    }

    let vsm_file = &args[1];
    let trace_type = if args.iter().any(|arg| arg == "-t") {
        TraceType::TraceStack
    }else {
        TraceType::No
    };


    let mut vsm = Vsm::new(trace_type);

    vsm.read_code(vsm_file).expect(&format!("File cannot be read filepath='{}'", vsm_file));

    vsm.exec_code().expect(&format!("Runtime error filepath='{}'", vsm_file));

    
}
