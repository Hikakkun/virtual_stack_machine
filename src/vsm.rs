use std::io;
use std::io::{BufRead, Read};
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
            _ => format!("SP = -1"),
        };
        println!("{:02}:{} {}", self.program_counter, instruction.to_string(), dsp);

        let stack_slice = &self.stack[0..=self.max_stack_pointer];
        stack_slice.iter().rev().enumerate().for_each(|(rev_index, value)|{
            let index = stack_slice.len() - rev_index-1;
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
            match instruction_ecec_reuslt {
                Ok(rc) => {
                    return_code =rc; 
                }
                Err(err) => {
                    return Err(err);
                }
            }

            if self.stack_pointer.is_some() {
                if self.stack_pointer.unwrap() > self.max_stack_pointer {
                    self.max_stack_pointer = self.stack_pointer.unwrap();
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

    fn stack_read(&self, address: Option<usize>) -> Result<i32, String> {

        match address  {
            Some(a) => {
                if a < self.stack.len(){
                    Ok(self.stack[a]) 
                }else{  
                    Err(format!("stack read error address = {}", a))
                }
            },
            None => {
                Err(format!("stack read address is None"))
            }
        }
    }

    fn stack_write(&mut self, address : Option<usize>, value : i32)-> Result<(), String>
    {
        match address  {
            Some(a) => {
                if a< self.stack.len(){
                    self.stack[a] = value;
                    Ok(())
                }else{  
                    Err(format!("stack write error address = {}", a))
                }
            },
            None => {
                Err(format!("stack write error address is None"))
            }
        }

    }


    fn stack_pointer_increment(&mut self){
        self.stack_pointer = match self.stack_pointer {
            Some(sp) => Some(sp + 1),
            None => Some(0),
        };
    }

    fn stack_pointer_decrement(&mut self) -> Result<(), String>  {
        self.stack_pointer = match  self.stack_pointer {
            Some(0) => None,
            Some(sp) => Some(sp-1),
            None => return Err("stack_pointer_decrement err stack_pointer is None".to_string())
        };
        Ok(())
    }

    fn base_register_read(&self, value : i32) -> Result<usize, String>
    {
        match value {
            0 => Ok(self.global_top_address),
            1 => Ok(self.frame_top_address) ,
            _ => Err(format!("error base register '{}'", value)),
        }        
    }

    fn perform_operation<F>(&mut self, operation_fn: F) -> Result<(), String>
    where
        F: Fn(i32, i32) -> i32,
    {
        let top_value = self.stack_read(self.stack_pointer)?;
        self.stack_pointer_decrement()?;
        let bottom_value = self.stack_read(self.stack_pointer)?;
        let result = operation_fn(bottom_value, top_value);
        self.stack_write(self.stack_pointer, result)?;
        Ok(())
    }    
    fn add_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn sub_fn(a: i32, b: i32) -> i32 {
        a - b
    }
    
    fn mul_fn(a: i32, b: i32) -> i32 {
        a * b
    }
    
    fn div_fn(a: i32, b: i32) -> i32 {
        a / b
    }
    
    fn mod_fn(a: i32, b: i32) -> i32 {
        a % b
    }
    
    fn eq_fn(a: i32, b: i32) -> i32 {
        (a == b) as i32
    }
    
    fn ne_fn(a: i32, b: i32) -> i32 {
        (a != b) as i32
    }
    
    fn gt_fn(a: i32, b: i32) -> i32 {
        (a > b) as i32
    }
    
    fn lt_fn(a: i32, b: i32) -> i32 {
        (a < b) as i32
    }
    
    fn ge_fn(a: i32, b: i32) -> i32 {
        (a >= b) as i32
    }
    
    fn le_fn(a: i32, b: i32) -> i32 {
        (a <= b) as i32
    }
    fn exec_instruction(&mut self, instruction : Instruction) -> Result<Option<i32>, String> {
        let mut return_code : Option<i32> = None;

        
        let operand1 = match instruction.operand[0] {
            Some(operand) => operand,
            None => -1,
        };
        let operand2 = match instruction.operand[1] {
            Some(operand) => operand,
            None => -1,
        };

        match instruction.operation_code {
            OperationCode::Isp => {
                if operand1 != 0{
                    self.stack_pointer = match self.stack_pointer {
                        Some(val) => Some((val as i32 + operand1) as usize),
                        None => Some((-1+operand1) as usize),
                    };                    
                }
            },
            OperationCode::La => {
                self.stack_pointer_increment();
                let base_register =  self.base_register_read(operand1)?;
                let value  = operand2 + base_register as i32;
                self.stack_write(self.stack_pointer, value)?;
            },

            OperationCode::Lv => {
                self.stack_pointer_increment();
                let base_register =  self.base_register_read(operand1)?;
                let address  = operand2 as usize + base_register;
                let value = self.stack_read(Some(address))?;
                self.stack_write(self.stack_pointer, value)?;
            }
            OperationCode::Lc => {
                self.stack_pointer_increment();
                self.stack_write(self.stack_pointer, operand1)?;
            },
            OperationCode::Li => {
                let address = self.stack_read(self.stack_pointer)?;
                let value = self.stack_read(Some(address as usize))?;

                self.stack_write(self.stack_pointer, value)?;
                
            },
            OperationCode::Dup => {
                let value = self.stack_read(self.stack_pointer)?;
                self.stack_pointer_increment();
                self.stack_write(self.stack_pointer, value)?;
            },
            OperationCode::Si => {
                let value = self.stack_read(self.stack_pointer)?;
                self.stack_pointer_decrement()?;
                let address = self.stack_read(self.stack_pointer)?;
                self.stack_pointer_decrement()?;
                self.stack_write(Some(address as usize), value)?;
            },
            OperationCode::Sv => {
                let base_register =  self.base_register_read(operand1)?;
                let address  = operand2 as usize + base_register;
                let value = self.stack_read(self.stack_pointer)?;
                self.stack_pointer_decrement()?;
                self.stack_write(Some(address as usize), value)?;
            },
            OperationCode::Sb => {
                let value = self.stack_read(self.stack_pointer)?;
                match operand1 {
                    0 => self.global_top_address = value as usize,
                    1 => self.frame_top_address = value as usize,
                    _ => {
                        return Err(format!("invalid instruction '{}'", instruction.to_string()));
                    }
                }

                self.stack_pointer_decrement()?;
            },
            OperationCode::B => {
                self.program_counter = ((self.program_counter as i32) + operand1) as usize;
            },
            OperationCode::Bz => {
                let value = self.stack_read(self.stack_pointer)?;
                if value == 0 {
                    self.program_counter = ((self.program_counter as i32) + operand1) as usize;
                }
                self.stack_pointer_decrement()?;
            },
            OperationCode::Call => {
                let stack_pointer = match self.stack_pointer {
                    Some(sp) => sp as i32,
                    None => -1,                    
                }; 
                
                let frame_address = stack_pointer+2;
                self.stack_write(Some(frame_address as usize), self.frame_top_address as i32)?;

                let pc_address = stack_pointer+3;
                self.stack_write(Some(pc_address as usize), self.program_counter as i32)?;
                self.frame_top_address = (stack_pointer + 1) as usize;
                self.program_counter = operand1 as usize;
            },
            OperationCode::Ret => {
                self.stack_pointer = Some(self.frame_top_address);


                let frame_address = self.stack_pointer.unwrap() + 1;
                let frame_top_address_value = self.stack_read(Some(frame_address as usize))?;
                self.frame_top_address = frame_top_address_value as usize;


                let pc_address = self.stack_pointer.unwrap() + 2;
                let program_counter_value = self.stack_read(Some(pc_address))?;
                self.program_counter = program_counter_value as usize;
            },

            OperationCode::Getc | OperationCode::Geti => {
                let stdin = io::stdin();
                let mut buffer = String::new();
                stdin.lock().read_line(&mut buffer).unwrap();

                self.stack_pointer_increment();
                match instruction.operation_code {
                    OperationCode::Getc=> {
                        let buffer_trim = buffer.trim();
                        if buffer_trim.len() != 1 {
                            return Err(format!("error getc input is one character inputcharacter='{}'", buffer_trim));
                        }
                        match buffer.chars().next() {
                            Some(input_char) => self.stack_write(self.stack_pointer, input_char as i32)?,
                            None => return Err(format!("error getc input is one character inputcharacter='{}'", buffer_trim)),
                        }
                    }
                    OperationCode::Geti =>{
                        let input_number = buffer.trim().parse::<i32>();
                        match input_number {
                            Ok(number) => {
                                self.stack_write(self.stack_pointer, number)?;
                            }
                            Err(err) => return Err(err.to_string())
                        }
                    }
                    _ => {}
                };

            }
            OperationCode::Putc | OperationCode::Puti => {
                let value =  self.stack_read(self.stack_pointer)?;
                self.stack_pointer_decrement()?;

                let print_str = match instruction.operation_code {
                    OperationCode::Putc => std::char::from_u32(value as u32).unwrap().to_string(),
                    OperationCode::Puti => value.to_string(),
                    _ => {"".to_string()},
                };
                print!("{}", print_str);
            },
            OperationCode::Add => self.perform_operation(Vsm::add_fn)?,
            OperationCode::Sub => self.perform_operation(Vsm::sub_fn)?,
            OperationCode::Mul => self.perform_operation(Vsm::mul_fn)?,
            OperationCode::Div => self.perform_operation(Vsm::div_fn)?,
            OperationCode::Mod => self.perform_operation(Vsm::mod_fn)?,
            OperationCode::Inv => {
                let value = - self.stack_read(self.stack_pointer)?;
                self.stack_write(self.stack_pointer, value)?;
            },
            OperationCode::Eq => self.perform_operation(Vsm::eq_fn)?,
            OperationCode::Ne => self.perform_operation(Vsm::ne_fn)?,
            OperationCode::Gt => self.perform_operation(Vsm::gt_fn)?,
            OperationCode::Lt => self.perform_operation(Vsm::lt_fn)?,
            OperationCode::Ge => self.perform_operation(Vsm::ge_fn)?,          
            OperationCode::Le => self.perform_operation(Vsm::le_fn)?,
            OperationCode::Exit => {
                return_code = match self.stack_read(self.stack_pointer) {
                    Ok(value) => Some(value),
                    Err(_) => Some(1),
                };
            }
        }

        Ok(return_code)
    }
}
