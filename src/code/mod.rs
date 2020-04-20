// nand2tetris - 124 page

/// Hackのアセンブリ言語のモーニックをバイナリコードへ変換する
/// ```
///      |      comp     | dest | jump
/// 1 1 1 a  c c c c  c c d d  d j j j
/// ```
pub struct Code {}

impl Code {
    /// destモーニックのバイナリコードを返す
    pub fn dest(monic: &str) -> Option<String> {
        let code = match monic {
            "" => "000",
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => return None
        };

        Some(code.to_string())
    }

    /// compモーニックのバイナリコードを返す
    pub fn comp(monic: &str) -> Option<String> {
        let code = match monic {
            // a=0のとき
            "0"   => concat!("0", "101010"),
            "1"   => concat!("0", "111111"),
            "-1"  => concat!("0", "111010"),
            "D"   => concat!("0", "001100"),
            "A"   => concat!("0", "110000"),
            "!D"  => concat!("0", "001101"),
            "!A"  => concat!("0", "110001"),
            "-D"  => concat!("0", "001111"),
            "-A"  => concat!("0", "110011"),
            "D+1" => concat!("0", "011111"),
            "A+1" => concat!("0", "110111"),
            "D-1" => concat!("0", "001110"),
            "A-1" => concat!("0", "110010"),
            "D+A" => concat!("0", "000010"),
            "D-A" => concat!("0", "010011"),
            "A-D" => concat!("0", "000111"),
            "D&A" => concat!("0", "000000"),
            "D|A" => concat!("0", "010101"),
            // a=1のとき
            "M"   => concat!("1", "110000"),
            "!M"  => concat!("1", "110001"),
            "-M"  => concat!("1", "110011"),
            "M+1" => concat!("1", "110111"),
            "M-1" => concat!("1", "110010"),
            "D+M" => concat!("1", "000010"),
            "D-M" => concat!("1", "010011"),
            "M-D" => concat!("1", "000111"),
            "D&M" => concat!("1", "000000"),
            "D|M" => concat!("1", "010101"),
            _ => return None
        };

        return Some(code.to_string())
    }

    /// jumpモーニックのバイナリコードを返す
    pub fn jump(monic: &str) -> Option<String> {
        let code = match monic {
            "" => "000",
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => return None
        };

        Some(code.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::Code;

    #[test]
    fn test_code_dest() {
        assert_eq!(&Code::dest(""   ).unwrap(), "000");
        assert_eq!(&Code::dest("M"  ).unwrap(), "001");
        assert_eq!(&Code::dest("D"  ).unwrap(), "010");
        assert_eq!(&Code::dest("MD" ).unwrap(), "011");
        assert_eq!(&Code::dest("A"  ).unwrap(), "100");
        assert_eq!(&Code::dest("AM" ).unwrap(), "101");
        assert_eq!(&Code::dest("AD" ).unwrap(), "110");
        assert_eq!(&Code::dest("AMD").unwrap(), "111");
        assert_eq!(Code::dest("a"), None);
    }

    #[test]
    fn test_code_jump() {
        assert_eq!(&Code::jump(""   ).unwrap(), "000");
        assert_eq!(&Code::jump("JGT"  ).unwrap(), "001");
        assert_eq!(&Code::jump("JEQ"  ).unwrap(), "010");
        assert_eq!(&Code::jump("JGE" ).unwrap(), "011");
        assert_eq!(&Code::jump("JLT"  ).unwrap(), "100");
        assert_eq!(&Code::jump("JNE" ).unwrap(), "101");
        assert_eq!(&Code::jump("JLE" ).unwrap(), "110");
        assert_eq!(&Code::jump("JMP").unwrap(), "111");
        assert_eq!(Code::jump("a"), None);
    }
}