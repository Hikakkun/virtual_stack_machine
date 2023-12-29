use core::num;
use std::{io, result, ffi::IntoStringError};
use std::io::{BufRead, Read, SeekFrom};
use std::io::{ Write};
use crate::code::{Code, Instruction, OperationCode};

#[derive(PartialEq)]
pub enum TraceType{
    No,
    TraceStack,
}

pub struct Vsm {
    code: Code,
    program_counter: usize,
    global_top_address: usize,
    frame_top_address: usize,
    stack: Vec<i32>,
    stack_pointer: Option<usize>,
    max_stack_pointer: usize,
    trace_type : TraceType
}

impl Vsm {
    pub fn new(trace_type : TraceType) -> Vsm {
        Vsm {
            code: Code::new(),
            program_counter: 0,
            global_top_address: 0,
            frame_top_address: 0,
            stack: vec![i32::default(); 1024],
            stack_pointer: None,
            max_stack_pointer: 0,
            trace_type : trace_type
        }
    }

    pub fn allocation_stack(&mut self, size: usize){
        self.stack = vec![i32::default(); size];
    }

    pub fn read_code(&mut self, file_path: &str)-> io::Result<()>{
        self.code.read(file_path)?;
        Ok(())
    }

    fn display_config(&self, instruction : Instruction){

        let dsp = match self.stack_pointer {
            Some(sp) => format!("SP = {}", sp),
            _ => format!("SP = None"),
        };
        println!("{:02}:{} {}", self.program_counter, instruction.to_string(), dsp);

        let _ = &self.stack[0..=self.max_stack_pointer].iter().rev().enumerate().for_each(|(rev_index, value)|{
            let index = self.stack.len() - rev_index-1;
            let b0 = match self.global_top_address == index {
                true => " <-B0",
                false => "",
            };
            let b1 = match self.frame_top_address == index {
                true => " <-B1",
                false => "",
            };
            
            let sp = match self.stack_pointer {
                Some(sp) if sp == index => " SP-> ",
                _ => "      ",
            };

            println!("{} Stack[{: >3}] {: >4}{}{}", sp, index, *value, b0, b1);
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
            
            if self.trace_type == TraceType::TraceStack {
                self.display_config(instruction);
                let stdin = io::stdin();
                let mut buffer = String::new();
                stdin.lock().read_line(&mut buffer).expect("Failed to read line");                
            }
        }
        Ok(())
    }

    fn stack_read<T>(&mut self, address : T)
    where
        T: Into<Option<usize>> + Into<Option<i32>> + Into<usize> + Into<i32>,
    {
        let usize_address : usize = address.into();
        return self.stack[usize_address];
    }

    fn stack_write(&mut self){

    }
    fn stack_pointer_increment(&mut self){
        self.stack_pointer = match self.stack_pointer {
            Some(sp) => Some(sp + 1),
            None => Some(0),
        };
    }
    fn stack_push<T: Into<Option<i32>>>(&mut self, value: T) {
        if let Some(val) = value.into().map(|v| v.into()) {
            self.stack.push(val);
    
            self.stack_pointer_increment();
        }
    }

    fn stack_pointer_decrement(&mut self) -> Result<(), String>  {
        self.stack_pointer = match  self.stack_pointer {
            Some(0) => None,
            Some(sp) => Some(sp-1),
            None => return Err("Cannot pop from an empty stack".to_string())
        };
        Ok(())
    }
    fn stack_pop(&mut self) -> Result<i32, String> {
        if let Some(value) = self.stack.pop() {
            self.stack_pointer_decrement()?;
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
                let prev_sp = match self.stack_pointer {
                    Some(sp) => sp as i32,
                    None => -1,
                };

                while prev_sp + instruction.operand[0].unwrap() == ((self.stack.len() as i32) -1) {
                    self.stack_push(i32::default());
                }

                self.stack_pointer_decrement()?;
            },
            OperationCode::La => {
                let base_register =  self.get_base_register(instruction.operand[0].unwrap())? as i32;
                let address = instruction.operand[1].unwrap();
                self.stack_push(base_register+address);
            },

            OperationCode::Lv => {
                let base_register =  self.get_base_register(instruction.operand[0].unwrap())?;
                let address = instruction.operand[1].unwrap() as usize;
                self.stack_push(self.stack[base_register+address]); 
            }
            OperationCode::Lc => {
                self.stack_push(instruction.operand[0]);
            },
            OperationCode::Li => {

                match self.stack_pointer {
                    Some(stack_pointer) => {
                        self.stack[stack_pointer] = self.stack[self.stack[stack_pointer] as usize];
                    },
                    _ => {
                        return Err(format!("stack pointer is None"));
                    }
                }
                
            },
            OperationCode::Dup => {
                let val = match self.stack_pointer {
                    Some(stack_pointer) => {
                        self.stack[stack_pointer]
                    },
                    _ => {
                        return Err(format!("stack pointer is None"));
                    }
                };


                self.stack_push(val);
            },
            OperationCode::Si => {
                let value = self.stack_pop()?;
                let address = self.stack_pop()?;
                self.stack[address as usize] = value;      
            },
            OperationCode::Sv => {
                let base_register =  self.get_base_register(instruction.operand[0].unwrap())?;
                let address = instruction.operand[1].unwrap() as usize;
                self.stack[base_register+address] = self.stack_pop()?;
            },
            OperationCode::Sb => {
                match instruction.operand[0].unwrap() {
                    0 => self.global_top_address = self.stack_pop()? as usize,
                    1 => self.frame_top_address = self.stack_pop()? as usize,
                    _ => {
                        return Err(format!("invalid instruction '{}'", instruction.to_string()));
                    }
                }
            },
            OperationCode::B => {
                self.program_counter = ((self.program_counter as i32) + instruction.operand[0].unwrap()) as usize;
            },
            OperationCode::Bz => {
                let value = self.stack_pop()?;
                if value == 0 {
                    self.program_counter = ((self.program_counter as i32) + instruction.operand[0].unwrap()) as usize;
                }
            },
            OperationCode::Call => {
                let prev_stak_pointer = self.stack_pointer;
                self.stack.push(0);
                self.stack_push(self.frame_top_address as i32);
                self.stack_push(self.program_counter as i32);
              
                self.frame_top_address = match self.stack_pointer {
                    Some(stack_pointer) => stack_pointer + 1,
                    None => 0,
                };
                self.program_counter = instruction.operand[0].unwrap() as usize;
            },
            OperationCode::Ret => {
                self.stack_pointer = Some(self.frame_top_address);
                self.frame_top_address = self.stack[self.stack_pointer.unwrap()+1] as usize;
                self.program_counter = self.stack[self.stack_pointer.unwrap()+2] as usize;
            },

            OperationCode::Getc => {
                let stdin = io::stdin();
                let mut buffer = [0; 1];
                stdin.lock().read_exact(&mut buffer).unwrap();

                let input_char = buffer[0] as char;

                self.stack_push(input_char as i32);
            },
            OperationCode::Geti => {
                let stdin = io::stdin();
                let mut buffer = String::new();
                stdin.lock().read_line(&mut buffer).unwrap();
                let input_number = buffer.trim().parse::<i32>();

                match input_number {
                    Ok(number) => self.stack_push(number),
                    Err(err) => return Err(err.to_string())
                }
            },
            OperationCode::Putc => {
                let value =  self.stack_pop()?;
                print!("{}", std::char::from_u32(value as u32).unwrap());
            },
            OperationCode::Puti => {
                print!("{}", self.stack_pop()?);
            },
            OperationCode::Add => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push(bottom_value+top_value);                
            },

            OperationCode::Sub => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push(bottom_value-top_value); 
            },
            OperationCode::Mul => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push(bottom_value*top_value); 
            },

            OperationCode::Div => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push(bottom_value/top_value); 
            },
            OperationCode::Mod => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push(bottom_value%top_value);            
            },
            OperationCode::Inv => {
                let value = self.stack_pop()?;
                self.stack_push(value);
            },
            OperationCode::Eq => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push((bottom_value==top_value) as i32);                 
            },
            OperationCode::Ne => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push((bottom_value!=top_value) as i32);                    
            },
            OperationCode::Gt => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push((bottom_value>top_value) as i32);                
            },
            OperationCode::Lt => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push((bottom_value<top_value) as i32);                  
            },
            OperationCode::Ge => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push((bottom_value>=top_value) as i32);                    
            },            
            OperationCode::Le => {
                let top_value = self.stack_pop()?;
                let bottom_value = self.stack_pop()?;
                self.stack_push((bottom_value<=top_value) as i32);                  
            },  
            OperationCode::Exit => {
                return_code = match self.stack_pop()  {
                    Ok(val) => Some(val),
                    Err(_) => None,
                };
            }
        }

        Ok(return_code)
    }
}
