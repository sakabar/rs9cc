#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum Tokenkind {
    TkReserved { str: String },
    TkNum { str: String, val: i32 },
    TkUnk { str: String },
    TkEof,
}

pub struct Tokenizer {
    tokens: Vec<Tokenkind>,
    index: usize,
}

impl Tokenizer {
    pub fn new(s: &String) -> Tokenizer {
        Tokenizer {
            tokens: Tokenizer::parse(&s),
            index: 0,
        }
    }

    pub fn parse(s: &String) -> Vec<Tokenkind> {
        let mut tokens: Vec<Tokenkind> = s
            // FIXME 雑実装
            .replace("+", " + ")
            .replace("-", " - ")
            .trim()
            .split_whitespace()
            .map(|token| {
                if token == "+" || token == "-" {
                    return Tokenkind::TkReserved {
                        str: String::from(token),
                    };
                } else {
                    let res = i32::from_str_radix(token, 10);
                    if res.is_err() {
                        return Tokenkind::TkUnk {
                            str: String::from(token),
                        };
                    }

                    return Tokenkind::TkNum {
                        str: String::from(token),
                        val: res.ok().unwrap(),
                    };
                }
            })
            .collect();

        // 末尾をTkEofにする
        tokens.push(Tokenkind::TkEof);
        return tokens;
    }

    fn cur(&self) -> &Tokenkind {
        &self.tokens[self.index]
    }

    pub fn cur_str(&self) -> &str {
        match self.cur() {
            Tokenkind::TkReserved { str: s }
            | Tokenkind::TkNum { str: s, val: _ }
            | Tokenkind::TkUnk { str: s } => s,
            _ => "",
        }
    }

    // 次のトークンが期待している記号のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    pub fn expect_op(&mut self, arg: &str) -> bool {
        match self.cur() {
            Tokenkind::TkReserved { str: s }
            | Tokenkind::TkNum { str: s, val: _ }
            | Tokenkind::TkUnk { str: s } => {
                if s == arg {
                    // 末尾には必ずTkEofがあるので、+=1してOK
                    self.index += 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    // 次のトークンが数値のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    pub fn expect_number(&mut self) -> Option<i32> {
        match self.cur() {
            &Tokenkind::TkNum { str: _, val: num } => {
                self.index += 1;
                Some(num)
            }
            _ => None,
        }
    }

    pub fn expect_eof(&self) -> bool {
        match self.cur() {
            &Tokenkind::TkEof => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        let actual = Tokenizer::parse(&String::from("42"));
        let expected = vec![
            Tokenkind::TkNum {
                str: String::from("42"),
                val: 42,
            },
            Tokenkind::TkEof,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_2() {
        let actual = Tokenizer::parse(&String::from("  42 - 21 + 10  "));
        let expected = vec![
            Tokenkind::TkNum {
                str: String::from("42"),
                val: 42,
            },
            Tokenkind::TkReserved {
                str: String::from("-"),
            },
            Tokenkind::TkNum {
                str: String::from("21"),
                val: 21,
            },
            Tokenkind::TkReserved {
                str: String::from("+"),
            },
            Tokenkind::TkNum {
                str: String::from("10"),
                val: 10,
            },
            Tokenkind::TkEof,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_3() {
        let actual = Tokenizer::parse(&String::from("5+20-4"));
        let expected = vec![
            Tokenkind::TkNum {
                str: String::from("5"),
                val: 5,
            },
            Tokenkind::TkReserved {
                str: String::from("+"),
            },
            Tokenkind::TkNum {
                str: String::from("20"),
                val: 20,
            },
            Tokenkind::TkReserved {
                str: String::from("-"),
            },
            Tokenkind::TkNum {
                str: String::from("4"),
                val: 4,
            },
            Tokenkind::TkEof,
        ];
        assert_eq!(actual, expected);
    }
}
