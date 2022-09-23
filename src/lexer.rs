// 2022/09/22 20:06:53 [error] 1036243#1036243: *3757623 no live upstreams while connecting to upstream, client: 192.168.11.2, server: , request: "GET /hoge HTTP/1.1", upstream: "http://localhost/flu/403.html", host: "192.168.11.1"

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(f64),
    Null, // Null
    WhiteSpace, // WhiteSpace
    Slash, // /
    Colon, // :
    LeftBracket, // [
    RightBracket, // ]
    Sharp, // #
    Asterisk, // *
    Period, // .
    DQuotaion, // "
}

pub struct Lexer<'a> {
    // Point to the first string being read
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

#[derive(Debug)]
// Exception
pub struct LexerError {
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                // 空白は今回は捨てるがデバッグ情報として使える(行、列)
                Token::WhiteSpace => {}
                _ => {
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }

    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    Slash, // /
    Colon, // :
    LeftBracket, // [
    RightBracket, // ]
    Sharp, // #
    Asterisk, // *
    Period, // .
    D_Quotaion, // "

    // return matched token
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        // get head of string
         match self.chars.peek() {
            Some(c) => match c {
                // if matched whitespace
                c if c.is_whitespace() => {
                    Ok(self.next_return_token(Token::WhiteSpace))
                }
                '/' => Ok(self.next_return_token(Token::Slash)),
                ':' => Ok(self.next_return_token(Token::Colon)),
                '[' => Ok(self.next_return_token(Token::LeftBracket)),
                ']' => Ok(self.next_return_token(Token::RightBracket)),
                '#' => Ok(self.next_return_token(Token::Sharp)),
                '*' => Ok(self.next_return_token(Token::Asterisk)),
                '.' => Ok(self.next_return_token(Token::Period)),
                '"' => Ok(self.next_return_token(Token::DQuotaion)),
            }
         }
    }

    /// nullの文字列をparseする
    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }
    /// (true|false)の文字列をparseする
    fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// 数字として使用可能な文字まで読み込む。読み込んだ文字列が数字(`f64`)としてParseに成功した場合Tokenを返す。
    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// 終端文字'\"'まで文字列を読み込む。UTF-16(\u0000~\uFFFF)や特殊なエスケープ文字(e.g. '\t','\n')も考慮する
    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// utf16のバッファが存在するならば連結しておく
    fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
        unimplemented!()
    }
}
