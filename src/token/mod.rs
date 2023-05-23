pub type TokenNumType = i32;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    PUNCT { op: char },        // 操作符，例如 + -
    NUM { val: TokenNumType }, // 数字
    EOF,                       // 文本文件的终止符
}

#[derive(Clone, Debug)]
pub struct Token {
    token_kind: TokenKind,
    loc: u64,
    len: u64,
}

impl Token {
    pub fn new(token_kind: TokenKind, loc: u64, len: u64) -> Self {
        Token {
            token_kind,
            loc,
            len,
        }
    }

    pub fn get_num_val(&self) -> TokenNumType {
        match self.token_kind {
            TokenKind::NUM { val } => val,
            _ => panic!("expect a num type token."),
        }
    }
}

pub fn tokenize(raw: &str) -> Vec<Token> {
    let mut res_vec = vec![];
    let mut ch_iter = raw.chars().into_iter().peekable();
    let mut loc = 0;

    while let Some(ch) = ch_iter.next() {
        match ch {
            ch if ch.is_whitespace() => {
                // need do nothing. loc++ will be done later.
            }
            first_digit_ch if first_digit_ch.is_ascii_digit() => {
                let mut digit_str = String::from(first_digit_ch);
                let mut len = 1;

                while let Some(next_ch) = ch_iter.peek() {
                    if next_ch.is_ascii_digit() {
                        digit_str.push({
                            len += 1;
                            loc += 1;
                            ch_iter.next().unwrap()
                        });
                    } else {
                        break;
                    }
                }

                let num = digit_str.parse::<TokenNumType>().unwrap();
                res_vec.push(Token::new(TokenKind::NUM { val: num }, loc, len));
            }
            op if ch == '+' || ch == '-' => {
                res_vec.push(Token::new(TokenKind::PUNCT { op }, loc, 1));
            }
            _ => {
                panic!("invaild token: {}{}", ch, ch_iter.collect::<String>());
            }
        }

        loc += 1;
    }

    for c in raw.chars() {
        match c {
            c if c.is_whitespace() => {
                continue;
            }
            c if c.is_digit(10) => {}
            c if c == '-' || c == '+' => {}
            _ => {}
        }
    }

    res_vec
}

mod tests {
    #[allow(unused)]
    use crate::token::{tokenize, TokenKind};

    #[test]
    pub fn tokenize_passed() {
        let tok_vec: Vec<super::Token> = tokenize("  12 + 34 - 5 ");

        assert_eq!(tok_vec.len(), 5);

        assert_eq!(
            tok_vec.get(0).unwrap().token_kind,
            TokenKind::NUM { val: 12 }
        );

        assert_eq!(
            tok_vec.get(1).unwrap().token_kind,
            TokenKind::PUNCT { op: '+' }
        );

        assert_eq!(
            tok_vec.get(2).unwrap().token_kind,
            TokenKind::NUM { val: 34 }
        );

        assert_eq!(
            tok_vec.get(3).unwrap().token_kind,
            TokenKind::PUNCT { op: '-' }
        );

        assert_eq!(
            tok_vec.get(4).unwrap().token_kind,
            TokenKind::NUM { val: 5 }
        );
    }
}
