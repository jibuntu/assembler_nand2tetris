// nand2tetris - 123 page

/// 主な機能は各アセンブリコマンドをその基本要素（フィールドとシンボル）に分解
/// することである。具体的には入力コードへのアクセスをカプセル化し、アセンブリ
/// 言語のコマンドを読み、それをパースし、コマンドの要素（フィールドと
/// シンボル）へ簡単にアクセスできるようなルーチンを提供する。さらに、空白文字
/// とコメントを削除する。
pub struct Parser {
    asm_lines: Vec<String>, // 不要なデータを除外した行のvector
    command: String, // 現在のコマンド
    count: usize, // 現在の行数
}

/// * `CommandType::A`は`@Xxx`を意味し、`Xxx`はシンボルか10進数の数値である
/// * `CommandType::C`は`dest=comp;jump`を意味する
/// * `CommandType::L`は疑似コマンドであり、`(Xxx)`を意味する。`Xxx`は
/// シンボルである
/// * `CommandType::None`は上記のどれにも該当しないことを意味する
#[derive(Debug, PartialEq)]
pub enum CommandType {
    A,
    C,
    L,
    None,
}

impl Parser {
    /// `Parser`を初期化
    /// `asm`はアセンブリ言語の文字列
    pub fn new(asm: String) -> Parser {
        let lines = asm.lines();
        let mut asm_lines = Vec::new();

        // 不要な行や空白を除外する
        for line in lines {
            let mut line = line;

            // コメントの削除
            let comment: Vec<_> = line.match_indices("//").collect();
            if comment.len() != 0 {
                line = line.get(..comment[0].0).unwrap();
            }
            
            // 空白の削除
            line = line.trim_matches(' ');

            if line.len() == 0 {
                continue;
            }

            asm_lines.push(line.to_string());
        }

        Parser {
            asm,
            asm_lines,
            command: String::new(),
            count: 0,
        }
    }

    /// 入力にまだコマンドが存在するか？
    pub fn has_more_commands(&self) -> bool {
        if self.count < self.asm_lines.len() {
            return true
        }
        return false
    }

    /// 入力から次のコマンドを読み、それを現在のコマンドにする。このルーチンは
    /// `has_more_commands()`が`true`の場合のみ呼ぶようにする。最初は現コマンド
    /// は空である。
    pub fn advance(&mut self) {
        self.command = self.asm_lines[self.count].to_string();
        self.count += 1;
    }

    /// 現コマンドの種類を返す
    /// * `CommandType::A`は`@Xxx`を意味し、`Xxx`はシンボルか10進数の数値である
    /// * `CommandType::C`は`dest=comp;jump`を意味する
    /// * `CommandType::L`は疑似コマンドであり、`(Xxx)`を意味する。`Xxx`は
    /// シンボルである
    /// * `CommandType::None`は上記のどれにも該当しないことを意味する
    pub fn command_type(&self) -> CommandType {
        if self.command.chars().next() == Some('@') {
            return CommandType::A
        }

        if self.command.find('=') != None || self.command.find(';') != None {
            return CommandType::C
        }

        if self.command.chars().next() == Some('(') 
           && self.command.chars().last() == Some(')') {
               return CommandType::L
        }

        return CommandType::None
    }

    /// 現コマンドの`@Xxx`または`(Xxx)`の`Xxx`を返す。Xxxはシンボルまたは10進数
    /// の数値である。このルーチンは`CommandType()`が`CommandType::A`または
    /// `CommandType::L`のときだけ呼ぶようにする
    pub fn symbol(&self) -> String {
        let mut chars = self.command.chars();
        match chars.next() {
            Some('@') => return self.command[1..].to_string(),
            Some('(') => return self.command[1..self.command.len()-1]
                                .to_string(),
            _ => "".to_string()
        }
    }

    /// 現C命令のdestモーニックを返す（候補として8つの可能性がある）。
    /// このルーチンは`command_type()`が`CommandType::C`のときだけ
    /// 呼ぶようにする
    pub fn dest(&self) -> String {
        if let Some(n) = self.command.find('=') {
            return self.command[..n].to_string()
        }

        return "".to_string()
    }

    /// 現C命令のcompモーニックを返す（候補として28つの可能性がある）。
    /// このルーチンは`command_type()`が`CommandType::C`のときだけ
    /// 呼ぶようにする
    pub fn comp(&self) -> String {
        if let Some(n) = self.command.find('=') {
            let command = &self.command[n+1..];
            if let Some(n) = command.find(';') {
                return command[..n].to_string();
            } else {
                return command.to_string()
            }
        } else if let Some(n) = self.command.find(';') {
            return self.command[..n].to_string();
        }
        
        return "".to_string()
    }

    /// 現C命令のjumpモーニックを返す（候補として8つの可能性がある）。
    /// このルーチンは`command_type()`が`CommandType::C`のときだけ
    /// 呼ぶようにする
    pub fn jump(&self) -> String {
        if let Some(n) = self.command.find(';') {
            return self.command[n+1..].to_string()
        }

        return "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::CommandType;

    #[test]
    fn test_parser_new() {
        let asm = r#"
        // test asm
          // test asm
        "#.to_string();
        let parser = Parser::new(asm);
        assert_eq!(parser.asm_lines.len(), 0);

        let asm = r#"
        // test asm
        @test
        @1 // test
        "#.to_string();
        let parser = Parser::new(asm);
        assert_eq!(parser.asm_lines.len(), 2);
        assert_eq!(parser.asm_lines, vec!["@test", "@1"]);
    }

    #[test]
    fn test_parser_has_more_commands() {
        let asm = r#""#.to_string();
        let parser = Parser::new(asm);
        assert_eq!(parser.asm_lines.len(), 0);
        assert_eq!(parser.has_more_commands(), false);

        let asm = r#"
        @test
        @test
        "#.to_string();
        let mut parser = Parser::new(asm);
        assert_eq!(parser.has_more_commands(), true);
        parser.advance();
        assert_eq!(parser.has_more_commands(), true);
        parser.advance();
        assert_eq!(parser.has_more_commands(), false);
    }

    #[test]
    fn test_test_parser_command_type() {
        let asm = r#"
        @Xxx
        dest=comp;jump
        dest=comp
        comp;jump
        (Xxx)
        aiueo
        "#.to_string();
        let mut parser = Parser::new(asm);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::A);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::C);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::C);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::C);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::L);
        parser.advance();
        assert_eq!(parser.command_type(), CommandType::None);
    }

    #[test]
    fn test_parser_symbol() {
        let asm = r#"
        @Xxx
        (Xxx)
        (X)
        @X
        "#.to_string();
        let mut parser = Parser::new(asm);
        parser.advance();
        assert_eq!(parser.symbol(), "Xxx".to_string());
        parser.advance();
        assert_eq!(parser.symbol(), "Xxx".to_string());
        parser.advance();
        assert_eq!(parser.symbol(), "X".to_string());
        parser.advance();
        assert_eq!(parser.symbol(), "X".to_string());
    }

    #[test]
    fn test_parser_dest() {
        let asm = r#"
        a=b
        a=b;c
        b;c
        "#.to_string();
        let mut parser = Parser::new(asm);
        parser.advance();
        assert_eq!(parser.dest(), "a");
        parser.advance();
        assert_eq!(parser.dest(), "a");
        parser.advance();
        assert_eq!(parser.dest(), "");
    }

    #[test]
    fn test_parser_comp() {
        let asm = r#"
        a=b
        a=b;c
        b;c
        "#.to_string();
        let mut parser = Parser::new(asm);
        parser.advance();
        assert_eq!(parser.comp(), "b");
        parser.advance();
        assert_eq!(parser.comp(), "b");
        parser.advance();
        assert_eq!(parser.comp(), "b");
    }

    #[test]
    fn test_parser_jump() {
        let asm = r#"
        a=b
        a=b;c
        b;c
        "#.to_string();
        let mut parser = Parser::new(asm);
        parser.advance();
        assert_eq!(parser.jump(), "");
        parser.advance();
        assert_eq!(parser.jump(), "c");
        parser.advance();
        assert_eq!(parser.jump(), "c");
    }
}

