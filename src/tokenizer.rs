use super::libc;
use std::iter;

type Coord = (usize, usize);

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Tokenkind {
    TkReserved { str: String, coord: Coord },
    TkNum { str: String, coord: Coord, val: i32 },
    TkUnk { str: String, coord: Coord },
    TkEof,
}

pub struct Tokenizer {
    program: String,
    tokens: Vec<Tokenkind>,
    index: usize,
}

impl Tokenizer {
    pub fn new(s: &String) -> Tokenizer {
        Tokenizer {
            program: s.to_string(),
            tokens: Tokenizer::parse(s),
            index: 0,
        }
    }

    pub fn parse(s: &String) -> Vec<Tokenkind> {
        let coord_row = 0;

        let mut coord_col: usize = 0;

        let mut rest_s: String = s.to_string();
        let mut ans_tokens: Vec<Tokenkind> = vec![];

        // 本当はchars()してからループを回したいが、そうするとバラバラの数字1文字ずつをまとめて1つの数字に復元する手間が発生する
        // 現時点の実装では、マルチバイトな文字列は存在しないと仮定する FIXME

        while !rest_s.is_empty() {
            // 空白は読み飛ばす
            if &rest_s[0..1] == " " {
                let token = &rest_s[0..1].to_string();
                coord_col += token.len();
                rest_s = rest_s[token.len()..].to_string();

                continue;
            }

            // "+" と "-" はトークナイズして読み進める
            if &rest_s[0..1] == "+" || &rest_s[0..1] == "-" {
                let token = rest_s[0..1].to_string();

                let coord = (coord_row, coord_col);
                rest_s = rest_s[token.len()..].to_string();
                coord_col += token.len();

                let ans = Tokenkind::TkReserved {
                    str: token,
                    coord: coord,
                };
                ans_tokens.push(ans);

                continue;
            }

            // 記号じゃなかったので数字パースしてみる
            let res = libc::strtol(&rest_s, 10);
            if res.is_err() {
                let token = rest_s;

                let ans = Tokenkind::TkUnk {
                    str: String::from(token),
                    coord: (coord_row, coord_col),
                };
                ans_tokens.push(ans);

                // どうせ使わないので消しておく
                // rest_s = String::new();
                // どうせrest_sを空文字としたことで次のループで止まるのでcoord_colは増やさない
                // coord_col += token.len() -1;

                // これ以上読み進めたくないので、あえてcontinueではなくbreak
                break;
            }

            let (val, parsed_rest) = res.ok().unwrap();

            // val.to_string() した値はプログラム中の文字列と異なる可能性があるので、
            // token文字列を求めるのは慎重に行っている
            let token = if parsed_rest == "" {
                rest_s.to_owned()
            } else {
                let rest_ind = rest_s.find(parsed_rest.as_str()).unwrap();
                rest_s[..rest_ind].to_string()
            };

            let coord = (coord_row, coord_col);
            rest_s = rest_s[token.len()..].to_string();
            coord_col += token.len();

            let ans = Tokenkind::TkNum {
                str: token,
                coord: coord,
                val: val as i32,
            };
            ans_tokens.push(ans);

            continue;
        }

        // 末尾をTkEofにする
        ans_tokens.push(Tokenkind::TkEof);
        return ans_tokens;
    }

    fn cur(&self) -> &Tokenkind {
        &self.tokens[self.index]
    }

    pub fn cur_str(&self) -> &str {
        match self.cur() {
            Tokenkind::TkReserved { str: s, coord: _ }
            | Tokenkind::TkNum {
                str: s,
                coord: _,
                val: _,
            }
            | Tokenkind::TkUnk { str: s, coord: _ } => s,
            Tokenkind::TkEof => "",
        }
    }

    // 次のトークンが期待している記号のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    pub fn expect_op(&mut self, arg: &str) -> bool {
        if self.cur_str() == arg {
            // 末尾には必ずTkEofがあるので、+=1してOK
            self.index += 1;
            return true;
        }

        false
    }

    // 次のトークンが数値のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    pub fn expect_number(&mut self) -> Option<i32> {
        match self.cur() {
            &Tokenkind::TkNum {
                str: _,
                coord: _,
                val: num,
            } => {
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

    pub fn get_coord(&self, token: &Tokenkind) -> Option<Coord> {
        match *token {
            Tokenkind::TkReserved { str: _, coord: c }
            | Tokenkind::TkNum {
                str: _,
                coord: c,
                val: _,
            }
            | Tokenkind::TkUnk { str: _, coord: c } => Some(c),
            Tokenkind::TkEof => None,
        }
    }

    fn error_at(&self, token_ind: usize, msg: &str) -> String {
        // FIXME ここ、0がベタ打ちになっている。
        let coord_row = 0;

        let ind = if token_ind <= self.tokens.len() - 1 {
            token_ind
        } else {
            self.tokens.len() - 1
        };
        let coord = self
            .get_coord(&self.tokens[ind])
            .unwrap_or((coord_row, self.program.len()));

        let spaces: String = iter::repeat(' ').take(coord.1).collect();

        format!("{}\n{}^ {}", self.program, spaces, msg)
    }

    pub fn error_at_cur(&self, msg: &str) -> String {
        self.error_at(self.index, msg)
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
                coord: (0, 0),
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
                coord: (0, 2),
                val: 42,
            },
            Tokenkind::TkReserved {
                str: String::from("-"),
                coord: (0, 5),
            },
            Tokenkind::TkNum {
                str: String::from("21"),
                coord: (0, 7),
                val: 21,
            },
            Tokenkind::TkReserved {
                str: String::from("+"),
                coord: (0, 10),
            },
            Tokenkind::TkNum {
                str: String::from("10"),
                coord: (0, 12),
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
                coord: (0, 0),
                val: 5,
            },
            Tokenkind::TkReserved {
                str: String::from("+"),
                coord: (0, 1),
            },
            Tokenkind::TkNum {
                str: String::from("20"),
                coord: (0, 2),
                val: 20,
            },
            Tokenkind::TkReserved {
                str: String::from("-"),
                coord: (0, 4),
            },
            Tokenkind::TkNum {
                str: String::from("4"),
                coord: (0, 5),
                val: 4,
            },
            Tokenkind::TkEof,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_4() {
        let actual = Tokenizer::parse(&String::from("5 + five"));
        let expected = vec![
            Tokenkind::TkNum {
                str: String::from("5"),
                coord: (0, 0),
                val: 5,
            },
            Tokenkind::TkReserved {
                str: String::from("+"),
                coord: (0, 2),
            },
            Tokenkind::TkUnk {
                str: String::from("five"),
                coord: (0, 4),
            },
            Tokenkind::TkEof,
        ];
        assert_eq!(actual, expected);
    }
}
