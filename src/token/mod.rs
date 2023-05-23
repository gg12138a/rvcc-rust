use std::collections::LinkedList;

pub type TokenNumType = i32;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    PUNCT { op: char },        // 操作符，例如 + -
    NUM { val: TokenNumType }, // 数字
    EOF,                       // 文本文件的终止符
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub loc: u64,
    pub len: u64,
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

pub fn tokenize(raw: &str) -> LinkedList<Token> {
    let mut tokens = LinkedList::new();
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
                tokens.push_back(Token::new(TokenKind::NUM { val: num }, loc, len));
            }
            op if ch == '+' || ch == '-' => {
                tokens.push_back(Token::new(TokenKind::PUNCT { op }, loc, 1));
            }
            _ => {
                panic!("invaild token: {}{}", ch, ch_iter.collect::<String>());
            }
        }

        loc += 1;
    }
    tokens
}

pub fn consume_one_num(v: &mut LinkedList<Token>) -> TokenNumType {
    let tok = v.pop_front().unwrap();

    if let TokenKind::NUM { val } = tok.token_kind {
        val
    } else {
        panic!("not NUM type token.")
    }
}

mod tests {
    #[allow(unused)]
    use crate::token::{tokenize, TokenKind};

    #[test]
    pub fn tokenize_passed() {
        let mut tokens = tokenize("  12 + 34 - 5 ");

        assert_eq!(tokens.len(), 5);

        assert_eq!(
            tokens.pop_front().unwrap().token_kind,
            TokenKind::NUM { val: 12 }
        );

        assert_eq!(
            tokens.pop_front().unwrap().token_kind,
            TokenKind::PUNCT { op: '+' }
        );

        assert_eq!(
            tokens.pop_front().unwrap().token_kind,
            TokenKind::NUM { val: 34 }
        );

        assert_eq!(
            tokens.pop_front().unwrap().token_kind,
            TokenKind::PUNCT { op: '-' }
        );

        assert_eq!(
            tokens.pop_front().unwrap().token_kind,
            TokenKind::NUM { val: 5 }
        );
    }
}
