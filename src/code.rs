use core::fmt;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum OperationCode {
    Isp,
    La,
    Lv,
    Lc,
    Li,
    Dup,
    Si,
    Sv,
    Sb,
    B,
    Bz,
    Call,
    Ret,
    Getc,
    Geti,
    Putc,
    Puti,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inv,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Exit,
}

impl fmt::Display for OperationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationCode::Isp => write!(f, "ISP"),
            OperationCode::La => write!(f, "LA"),
            OperationCode::Lv => write!(f, "LV"),
            OperationCode::Lc => write!(f, "LC"),
            OperationCode::Li => write!(f, "LI"),
            OperationCode::Dup => write!(f, "DUP"),
            OperationCode::Si => write!(f, "SI"),
            OperationCode::Sv => write!(f, "SV"),
            OperationCode::Sb => write!(f, "SB"),
            OperationCode::B => write!(f, "B"),
            OperationCode::Bz => write!(f, "BZ"),
            OperationCode::Call => write!(f, "CALL"),
            OperationCode::Ret => write!(f, "RET"),
            OperationCode::Getc => write!(f, "GETC"),
            OperationCode::Geti => write!(f, "GETI"),
            OperationCode::Putc => write!(f, "PUTC"),
            OperationCode::Puti => write!(f, "PUTI"),
            OperationCode::Add => write!(f, "ADD"),
            OperationCode::Sub => write!(f, "SUB"),
            OperationCode::Mul => write!(f, "MUL"),
            OperationCode::Div => write!(f, "DIV"),
            OperationCode::Mod => write!(f, "MOD"),
            OperationCode::Inv => write!(f, "INV"),
            OperationCode::Eq => write!(f, "EQ"),
            OperationCode::Ne => write!(f, "NE"),
            OperationCode::Gt => write!(f, "GT"),
            OperationCode::Lt => write!(f, "LT"),
            OperationCode::Ge => write!(f, "GE"),
            OperationCode::Le => write!(f, "LE"),
            OperationCode::Exit => write!(f, "EXIT"),
        }
    }
}

impl FromStr for OperationCode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "ISP" => Ok(OperationCode::Isp),
            "LA" => Ok(OperationCode::La),
            "LV" => Ok(OperationCode::Lv),
            "LC" => Ok(OperationCode::Lc),
            "LI" => Ok(OperationCode::Li),
            "DUP" => Ok(OperationCode::Dup),
            "SI" => Ok(OperationCode::Si),
            "SV" => Ok(OperationCode::Sv),
            "SB" => Ok(OperationCode::Sb),
            "B" => Ok(OperationCode::B),
            "BZ" => Ok(OperationCode::Bz),
            "CALL" => Ok(OperationCode::Call),
            "RET" => Ok(OperationCode::Ret),
            "GETC" => Ok(OperationCode::Getc),
            "GETI" => Ok(OperationCode::Geti),
            "PUTC" => Ok(OperationCode::Putc),
            "PUTI" => Ok(OperationCode::Puti),
            "ADD" => Ok(OperationCode::Add),
            "SUB" => Ok(OperationCode::Sub),
            "MUL" => Ok(OperationCode::Mul),
            "DIV" => Ok(OperationCode::Div),
            "MOD" => Ok(OperationCode::Mod),
            "INV" => Ok(OperationCode::Inv),
            "EQ" => Ok(OperationCode::Eq),
            "NE" => Ok(OperationCode::Ne),
            "GT" => Ok(OperationCode::Gt),
            "LT" => Ok(OperationCode::Lt),
            "GE" => Ok(OperationCode::Ge),
            "LE" => Ok(OperationCode::Le),
            "EXIT" => Ok(OperationCode::Exit),
            _ => Err("Invalid operation code"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    pub operation_code: OperationCode,
    pub operand: [Option<i32>; 2],
}

// MyStructにDisplayトレイトを実装
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut operand_str = self
            .operand
            .iter()
            .filter_map(|op| op.map(|val| val.to_string()))
            .collect::<Vec<String>>()
            .join(" ");
        if !operand_str.is_empty() {
            operand_str = " ".to_string() + &operand_str;
        }
        // 構造体の各フィールドをフォーマットして書き込む
        write!(f, "{}{}", self.operation_code.to_string(), operand_str)
    }
}

pub struct Code {
    operand_size_map: HashMap<OperationCode, usize>,
    instruction_vec: Vec<Instruction>,
}

impl Code {

    pub fn get_instruction(&self, program_counter : usize) -> Instruction {
        self.instruction_vec[program_counter].clone()
    }
    pub fn len(&self) -> usize {
        self.instruction_vec.len()
    }
    pub fn new() -> Code {
        let operand_size_map_init = [
            (OperationCode::Isp, 1),
            (OperationCode::La, 2),
            (OperationCode::Lv, 2),
            (OperationCode::Lc, 1),
            (OperationCode::Li, 0),
            (OperationCode::Dup, 0),
            (OperationCode::Si, 0),
            (OperationCode::Sv, 2),
            (OperationCode::Sb, 1),
            (OperationCode::B, 1),
            (OperationCode::Bz, 1),
            (OperationCode::Call, 1),
            (OperationCode::Ret, 0),
            (OperationCode::Getc, 0),
            (OperationCode::Geti, 0),
            (OperationCode::Putc, 0),
            (OperationCode::Puti, 0),
            (OperationCode::Add, 0),
            (OperationCode::Sub, 0),
            (OperationCode::Mul, 0),
            (OperationCode::Div, 0),
            (OperationCode::Mod, 0),
            (OperationCode::Inv, 0),
            (OperationCode::Eq, 0),
            (OperationCode::Ne, 0),
            (OperationCode::Gt, 0),
            (OperationCode::Lt, 0),
            (OperationCode::Ge, 0),
            (OperationCode::Le, 0),
            (OperationCode::Exit, 0),
        ];

        Code {
            operand_size_map: HashMap::from(operand_size_map_init),
            instruction_vec: Vec::new(),
        }
    }

    fn remove_comments(input: &str) -> String {
        let comment_regex = Regex::new(r"//.*$").unwrap();
        comment_regex.replace_all(input, "").to_string()
    }

    pub fn read(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let command = match line {
                Ok(command) => command,
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                    return Err(err.into());
                }
            };

            let command_remove_comments = Code::remove_comments(&command);
            let command_parse = command_remove_comments
                .split_whitespace()
                .collect::<Vec<_>>();

            if !command_parse.is_empty() {
                let operation_str = command_parse[0];
                let operation_code_result = operation_str.to_string().parse::<OperationCode>();
                if operation_code_result.is_err() {
                    eprintln!("Error parsing OperationCode: {}", operation_str);
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid OperationCode",
                    ));
                }
                let operation_code = operation_code_result.unwrap();

                if let Some(operand_size) = self.operand_size_map.get(&operation_code) {
                    if operand_size + 1 != command_parse.len() {
                        eprintln!("command '{}'", command_remove_comments);
                        eprintln!(
                            "'{}' has '{}' arguments but '{}' input arguments",
                            operation_code.to_string(),
                            operand_size,
                            command_parse.len() - 1
                        );
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid OperationCode",
                        ));
                    }

                    fn parse_operand(command_parse: &[&str], index: usize) -> Result<Option<i32>, std::num::ParseIntError> {
                        let inner_parse_operand = |value: &str| -> Result<Option<i32>, std::num::ParseIntError> {
                            value.parse::<i32>().map(Some)
                        };
                    
                        if command_parse.len() > index {
                            inner_parse_operand(command_parse[index])
                        } else {
                            Ok(None)
                        }
                    }

                    let operand0 = parse_operand(&command_parse, 1).unwrap();
                    let operand1 = parse_operand(&command_parse, 2).unwrap();

                    self.append_instruction(operation_code, operand0, operand1);
                } else {
                    eprintln!(
                        "OperationCode operand_size is not defined: {}",
                        operation_str
                    );
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid OperationCode",
                    ));
                }
            }
        }

        Ok(())
    }
    pub fn write(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        self.instruction_vec
            .iter()
            .try_for_each(|instruction| writeln!(file, "{}", instruction))?;

        Ok(())
    }
    pub fn append_instruction(
        &mut self,
        operation_code: OperationCode,
        operand0: Option<i32>,
        operand1: Option<i32>,
    ) {
        self.instruction_vec.push(Instruction {
            operation_code: operation_code,
            operand: [operand0, operand1],
        });
    }

    pub fn set_instruction(
        &mut self,
        index: usize,
        operation_code: OperationCode,
        operand0: Option<i32>,
        operand1: Option<i32>,
    ) {
        self.instruction_vec[index] = Instruction {
            operation_code: operation_code,
            operand: [operand0, operand1],
        };
    }
    pub fn print(&self) {
        self.instruction_vec
            .iter()
            .for_each(|instruction| println!("{}", instruction));
    }
}
