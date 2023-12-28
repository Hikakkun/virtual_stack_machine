use core::num;
use std::{io, result, ffi::IntoStringError};
use std::io::{BufRead, Read};
use std::io::{ Write};
use crate::code::{Code, Instruction, OperationCode};

#[derive(PartialEq)]
pub enum TraceType{
    No,
    TraceMemory,
}

pub struct Vsm {
    code: Code,
    program_counter: usize,
    global_top_address: usize,
    frame_top_address: usize,
    memory: Vec<i32>,
    base_register: Vec<i32>,
    trace_type : TraceType
}

impl Vsm {
    pub fn new(trace_type : TraceType) -> Vsm {
        Vsm {
            code: Code::new(),
            program_counter: 0,
            global_top_address: 0,
            frame_top_address: 0,
            memory: Vec::new(),
            base_register: Vec::new(),
            trace_type : trace_type
        }
    }

    pub fn read_code(&mut self, file_path: &str)-> io::Result<()>{
        self.code.read(file_path)?;
        Ok(())
    }

    fn display_config(&self, instruction : Instruction){
        println!("{:02}:{}", self.program_counter, instruction.to_string());
        self.memory.iter().rev().enumerate().for_each(|(rev_index, value)|{
            let index = self.memory.len() - rev_index-1;
            let b0 = if self.global_top_address == index {
                " <-B0"
            }else{
                ""
            };

            let b1 = if self.frame_top_address == index {
                " <-B1"
            }else{
                ""
            };

            println!("M[{:03}] {:04}{}{}", index, value, b0, b1);
        });  
        println!("\n");
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

            //self.display_config(instruction);
            match instruction_ecec_reuslt {
                Ok(rc) => {
                    return_code =rc; 
                }
                Err(err) => {
                    return Err(err);
                }
            }
            
            if self.trace_type == TraceType::TraceMemory {
                self.display_config(instruction);
                let stdin = io::stdin();
                let mut buffer = String::new();
                stdin.lock().read_line(&mut buffer).expect("Failed to read line");                
            }
        }
        Ok(())
    }

    fn memory_pop(&mut self) -> Result<i32, String> {
        if let Some(value) = self.memory.pop() {
            Ok(value)
        }else{
            Err("Cannot pop from an empty stack".to_string())
        }
    }

    fn get_base_register(&self, value : i32) -> Result<usize, String>{
        match value {
            0 => Ok(self.global_top_address),
            1 => Ok(self.frame_top_address) ,
            _ => Err(format!("invalid instruction '{}'", value))
        }        
    }
    fn exec_instruction(&mut self, instruction : Instruction) -> Result<Option<i32>, String> {
        let mut return_code : Option<i32> = None;

        match instruction.operation_code {
            OperationCode::Isp => {
                std::iter::repeat(0)
                .take(instruction.operand[0].unwrap() as usize)
                .for_each(|_| self.memory.push(i32::default()));
            },
            OperationCode::La => {
                let base_register =  self.get_base_register(instruction.operand[0].unwrap())? as i32;
                let address = instruction.operand[1].unwrap();
                self.memory.push(base_register+address);
            },

            OperationCode::Lv => {
                let base_register =  self.get_base_register(instruction.operand[0].unwrap())?;
                let address = instruction.operand[1].unwrap() as usize;
                self.memory.push(self.memory[base_register+address]); 
            }
            OperationCode::Lc => {
                self.memory.push(instruction.operand[0].unwrap());
            },
            OperationCode::Li => {
                let address = self.memory.last().unwrap();
                let value = self.memory[*address as usize];
                self.memory_pop()?;
                self.memory.push(value);
            },
            OperationCode::Dup => {
                self.memory.push(*self.memory.last().unwrap());
            },
            OperationCode::Si => {
                let value = self.memory_pop()?;
                let address = self.memory_pop()?;
                self.memory[address as usize] = value;        
            },
            OperationCode::Sv => {
                let base_register =  self.get_base_register(instruction.operand[0].unwrap())?;
                let address = instruction.operand[1].unwrap() as usize;
                self.memory[base_register+address] = self.memory_pop()?;
            },
            OperationCode::Sb => {
                match instruction.operand[0].unwrap() {
                    0 => self.global_top_address = self.memory_pop()? as usize,
                    1 => self.frame_top_address = self.memory_pop()? as usize,
                    _ => {
                        return Err(format!("invalid instruction '{}'", instruction.to_string()));
                    }
                }
            },
            OperationCode::B => {
                self.program_counter = ((self.program_counter as i32) + instruction.operand[0].unwrap()) as usize;
            },
            OperationCode::Bz => {
                let value = self.memory_pop()?;
                if value == 0 {
                    self.program_counter += instruction.operand[0].unwrap() as usize;
                }
            },
            OperationCode::Call => {
                let top_address = self.memory.len()-1;
                self.memory[top_address] = self.program_counter as i32;
                self.memory[top_address-1] = self.frame_top_address as i32;
                self.frame_top_address = top_address -3 + 1;
                self.program_counter = instruction.operand[0].unwrap() as usize;
            },
            OperationCode::Ret => {
                while self.memory.len()-1 -2 != self.frame_top_address {
                    self.memory_pop()?;
                }
                self.program_counter = self.memory_pop()? as usize;
                self.frame_top_address = self.memory_pop()? as usize;
            },

            OperationCode::Getc => {
                let stdin = io::stdin();
                let mut buffer = [0; 1];
                stdin.lock().read_exact(&mut buffer).unwrap();

                let input_char = buffer[0] as char;

                self.memory.push(input_char as i32);
            },
            OperationCode::Geti => {
                let stdin = io::stdin();
                let mut buffer = String::new();
                stdin.lock().read_line(&mut buffer).unwrap();
                let input_number = buffer.trim().parse::<i32>();

                match input_number {
                    Ok(number) => self.memory.push(number),
                    Err(err) => return Err(err.to_string())
                }
            },
            OperationCode::Putc => {
                let value =  self.memory_pop()?;
                print!("{}", std::char::from_u32(value as u32).unwrap());
            },
            OperationCode::Puti => {
                print!("{}", self.memory_pop()?);
            },
            OperationCode::Add => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push(bottom_value+top_value);
                
            },

            OperationCode::Sub => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push(bottom_value-top_value);
            },
            OperationCode::Mul => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push(bottom_value*top_value);
            },

            OperationCode::Div => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push(bottom_value/top_value);
            },
            OperationCode::Mod => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push(bottom_value%top_value);                
            },
            OperationCode::Inv => {
                let value = self.memory_pop()?;
                self.memory.push(-value);
            },
            OperationCode::Eq => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push((bottom_value==top_value) as i32);                       
            },
            OperationCode::Ne => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push((bottom_value!=top_value) as i32);                   
            },
            OperationCode::Gt => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push((bottom_value>top_value) as i32);                   
            },
            OperationCode::Lt => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push((bottom_value<top_value) as i32);                   
            },
            OperationCode::Ge => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push((bottom_value>=top_value) as i32);                   
            },            
            OperationCode::Le => {
                let top_value = self.memory_pop()?;
                let bottom_value = self.memory_pop()?;
                self.memory.push((bottom_value<=top_value) as i32);                   
            },  
            OperationCode::Exit => {
                return_code = Some(self.memory.pop().unwrap_or(0));
            }
        }

        Ok(return_code)
    }
}
