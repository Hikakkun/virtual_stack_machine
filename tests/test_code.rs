#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::Path;
    use virtual_stack_machine::code::Code;

    fn write_to_file_for_test(file_path: &str, content: &str) -> io::Result<()> {
        let path = Path::new(file_path);
        let mut file = File::create(path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_read_code_ok() {
        let file_path = "tests/read_code_ok.txt";
        let file_contents = r#"
        LC 1
        LC 2
        ADD 
        
        LC 3
        
        LC 18
        
        LC 4
        LC 9
        ADD
        
        DIV
        
        SUB
        
        MUL
        
        PUTI
        
        LC 10 
        PUTC
        
        EXIT

        "#;

        write_to_file_for_test(file_path, file_contents).unwrap();
        let mut code = Code::new();
        assert!(code.read(file_path).is_ok());

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_code_ng() {
        let file_path = "tests/read_code_ng.txt";
        let file_contents = r#"
        LC
        "#;

        write_to_file_for_test(file_path, file_contents).unwrap();
        let mut code = Code::new();
        assert!(code.read(file_path).is_err());

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_code_comment() {
        let file_path = "tests/read_code_comment.txt";
        let file_contents = r#"
        ISP 3
        //変数領域の確保 (&a=0, &b=1, &c=2)
        
        LA 0 0
        LC 3
        SI
        // a = 3
        
        LA 0 1
        LC 4
        SI
        //b = 4
        
        LA 0 2 
        // c のアドレスのロード
        
        LV 0 0
        LV 0 0
        MUL
        // a*a
        
        LV 0 1
        LV 0 1
        MUL
        // b*b
        
        ADD
        //a*a + b*b
        
        SI
        //c = a*a + b*b
        
        LV 0 2 
        PUTI 
        //print(c)
        
        LC 10 
        PUTC
        // /n
        
        EXIT
        "#;

        write_to_file_for_test(file_path, file_contents).unwrap();
        let mut code = Code::new();
        assert!(code.read(file_path).is_ok());

        fs::remove_file(file_path).unwrap();
    }
}
