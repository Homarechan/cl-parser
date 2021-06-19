struct Parser {
    src: String,
    point: usize,
    length: usize,
}

#[derive(Debug)]
pub struct CommandTree {
    cmd: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    EmptySource,
    PointOutOfBound(String),
    NoCommandGiven,
}

#[derive(PartialEq)]
pub enum ParseMode {
    Exactly,
    Fuzzy,
}

impl Parser {
    fn new(src_: String) -> Result<Self, Error> {
        let src = src_.trim().to_string();
        // Empty source should return error
        if src.clone().is_empty() {
            return Err(Error::EmptySource);
        }

        Ok(Self {
            src: src.clone(),
            point: 0,
            length: src.len(),
        })
    }

    fn char_now(&self) -> Result<char, Error> {
        let c_: Option<char> = self.src.chars().nth(self.point);
        if let Some(c) = c_ {
            Ok(c)
        } else {
            Err(Error::PointOutOfBound(format!(
                "{}th charactor doesn't found",
                self.point
            )))
        }
    }

    fn skip_white(&mut self) -> Result<(), Error> {
        // This can't be an error because whitespace at last was stripped
        while self.point < self.length {
            if self.char_now()? == ' ' {
                self.point += 1;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn get_cmd(&mut self, mode: ParseMode) -> Result<String, Error> {
        let mut cmd = String::new();

        while self.point < self.length {
            let c = self.char_now()?;
            match c {
                'A'..='Z' | 'a'..='z' => {
                    cmd.push(c);
                    self.point += 1;
                }
                _ => break,
            }
        }

        if cmd.is_empty() & (mode == ParseMode::Exactly) {
            Err(Error::NoCommandGiven)
        } else {
            Ok(cmd)
        }
    }

    fn get_args(&mut self) -> Result<Vec<String>, Error> {
        let mut args: Vec<String> = Vec::new();

        while self.point < self.length {
            self.skip_white()?;

            let mut arg: String = String::new();

            while (self.point < self.length) && (self.char_now()? != ' ')  {
                arg.push(self.char_now()?);
                self.point += 1;
            }

            args.push(arg);
        }

        Ok(args)
    }

    fn parse(&mut self, mode: ParseMode) -> Result<CommandTree, Error> {
        let cmd: String = self.get_cmd(mode)?;
        let args: Vec<String> = self.get_args()?;

        Ok(CommandTree { cmd, args })
    }
}

pub fn parse(src: String, mode: ParseMode) -> Result<CommandTree, Error> {
    let mut parser = Parser::new(src)?;
    let result = parser.parse(mode)?;
    Ok(result)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn empty_source() -> Result<(), String> {
        if let Ok(_) = Parser::new("".to_string()) {
            Err("Empty source have to be an error".to_string())
        } else {
            Ok(())
        }
    }

    #[test]
    fn get_cmd_not_found() -> Result<(), String> {
        let mut parser = Parser::new(":abc".to_string()).unwrap();

        if let Ok(_) = parser.get_cmd(ParseMode::Exactly) {
            Err("This parse have to be an error".to_string())
        } else {
            Ok(())
        }
    }

    #[test]
    fn get_cmd_fuzzy() -> Result<(), String> {
        let mut parser = Parser::new(":abc".to_string()).unwrap();

        if let Ok(_) = parser.get_cmd(ParseMode::Fuzzy) {
            Ok(())
        } else {
            Err("This parse have to be correct".to_string())
        }
    }
}
