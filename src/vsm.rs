use std::{io, result};
use crate::code::{Code, Instruction, OperationCode};
pub struct Vsm {
    code: Code,
    program_counter: usize,
    global_top_address: i32,
    frame_top_address: i32,
    memory: Vec<i32>,
    base_register: Vec<i32>
}

impl Vsm {
    pub fn new() -> Vsm {
        Vsm {
            code: Code::new(),
            program_counter: 0,
            global_top_address: 0,
            frame_top_address: 0,
            memory: Vec::new(),
            base_register: Vec::new(),
        }
    }

    pub fn read_code(&mut self, file_path: &str)-> io::Result<()>{
        self.code.read(file_path)?;
        Ok(())
    }

    pub fn exec_code(&mut self) -> Result<(), String>{

        let mut return_code : Option<i32> = None;

        while return_code.is_none() {
            
            if self.code.len() <= self.program_counter {
                return Err(format!("PC out of range (PC={})", self.program_counter))
            }
    
            let instruction = self.code.get_instruction(self.program_counter);
            self.program_counter += 1;
    
            let instruction_ecec_reuslt = self.exec_instruction(instruction);


            match instruction_ecec_reuslt {
                Ok(rc) => {
                    return_code =rc; 
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    fn exec_instruction(&mut self, instruction : Instruction) -> Result<Option<i32>, String> {
        let mut return_code : Option<i32> = None;

        match instruction.operation_code {
            OperationCode::Lc => {
                self.memory.push(instruction.operand[0].unwrap());
            },

            OperationCode::Putc => {
                let value =  self.memory.pop().unwrap();
                print!("{}", std::char::from_u32(value as u32).unwrap());
            },
            OperationCode::Puti => {
                print!("{}", self.memory.pop().unwrap());
            },
            OperationCode::Add => {
                let top_value = self.memory.pop().unwrap();
                let bottom_value = self.memory.pop().unwrap();
                self.memory.push(bottom_value+top_value);
                
            },

            OperationCode::Sub => {
                let top_value = self.memory.pop().unwrap();
                let bottom_value = self.memory.pop().unwrap();
                self.memory.push(bottom_value-top_value);
            },
            OperationCode::Mul => {
                let top_value = self.memory.pop().unwrap();
                let bottom_value = self.memory.pop().unwrap();
                self.memory.push(bottom_value*top_value);
            },

            OperationCode::Div => {
                let top_value = self.memory.pop().unwrap();
                let bottom_value = self.memory.pop().unwrap();
                self.memory.push(bottom_value/top_value);
            },


            OperationCode::Exit => {
                return_code = Some(self.memory.pop().unwrap_or(0));
            }
            _ => {
                return Err(format!("invalid instruction '{}'", instruction.to_string()));
            }
        }

        Ok(return_code)
    }
}
