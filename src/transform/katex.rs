use std::{
    process::{ExitStatus, Stdio},
    str::Utf8Error,
};

use itertools::Itertools;
use tokio::{io::AsyncWriteExt, process::Command, time::Instant};
use tracing::trace;

#[derive(thiserror::Error, Debug)]
pub enum KatexError {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Error parsing UTF-8 from CLI output: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("Executing dot failed with status code {0}. stderr:\n{1}")]
    CmdFailed(ExitStatus, String),
}

#[tracing::instrument(skip_all)]
pub async fn transform_katex(source: &str) -> Result<String, KatexError> {
    let mut cmd = Command::new("katex");
    cmd.arg("--trust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    trace!(?cmd, "executing katex");

    let start = Instant::now();
    let mut proc = cmd.spawn()?;
    trace!(%source, "writing source code");
    let mut stdin = proc.stdin.take().unwrap();
    stdin.write(source.as_bytes()).await?;
    drop(stdin);

    let result = proc.wait_with_output().await?;

    let status = result.status;

    trace!(?status, elapsed = ?start.elapsed(), "command exited");

    let svg = String::from_utf8(result.stdout).unwrap();
    let log = String::from_utf8_lossy(&result.stderr);

    if !status.success() {
        return Err(KatexError::CmdFailed(status, log.into()));
    }

    Ok(svg)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedKatexable {
    Text(String),
    BlockMath(String),
    InlineMath(String),
}

pub fn find_katex(s: &str) -> Vec<ParsedKatexable> {
    use Token::*;

    let chars: Vec<char> = s.chars().collect::<Vec<_>>();
    let tokens = tokenize(&chars);
    let mut pos = &tokens[..];
    let mut out = vec![];

    macro_rules! terminate_dollar {
        ($cap:expr, $constructor:expr) => {
            let rest = &pos[1..];
            match rest.iter().position(|t| t == &$cap) {
                Some(i) => {
                    // Found the cap to terminate
                    let between = &rest[..i];
                    let after = &rest[i + 1..];
                    out.push($constructor(between.iter().map(|t| t.to_string()).join("")));
                    pos = after;
                }
                None => {
                    // Did not find terminator, assume it's simple text
                    out.push(ParsedKatexable::Text(format!(
                        "{}{}",
                        $cap,
                        rest.iter().map(|t| t.to_string()).join("")
                    )));
                    break;
                }
            }
        };
    }

    loop {
        match pos {
            [Text(t), ..] => {
                out.push(ParsedKatexable::Text(t.clone()));
                pos = &pos[1..];
            }
            &[SingleDollar, ..] => {
                terminate_dollar!(SingleDollar, ParsedKatexable::InlineMath);
            }
            &[DoubleDollar, ..] => {
                terminate_dollar!(DoubleDollar, ParsedKatexable::BlockMath);
            }
            &[] => break,
        }
    }

    if out.len() <= 1 {
        return out;
    }

    let mut pos = &out[..];
    let mut out2 = vec![];
    while let Some((n, rest)) = pos.split_first() {
        match (n, rest.split_first()) {
            Some((ParsedKatexable::Text(l), Some((ParsedKatexable::Text(r), rest)))) => {
                out2.push(ParsedKatexable::Text(format!("{}{}", l, r)));
                pos = rest;
            }
            _ => {
                
                pos = rest;
            }
        }
    }
    return out2;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    SingleDollar,
    DoubleDollar,
    Text(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::SingleDollar => write!(f, "$")?,
            Token::DoubleDollar => write!(f, "$$")?,
            Token::Text(t) => write!(f, "{t}")?,
        }
        Ok(())
    }
}

fn tokenize(mut s: &[char]) -> Vec<Token> {
    use Token::*;
    let mut tokens = vec![];
    let mut buf = String::new();
    loop {
        match s {
            &['$', '$', ..] => {
                if !buf.is_empty() {
                    tokens.push(Text(buf));
                    buf = String::new();
                }
                tokens.push(DoubleDollar);
                s = &s[2..];
            }
            &['$', ..] => {
                if !buf.is_empty() {
                    tokens.push(Text(buf));
                    buf = String::new();
                }
                tokens.push(SingleDollar);
                s = &s[1..];
            }
            &['\\', '$', '$', ..] => {
                buf.extend("$$".chars());
                s = &s[3..];
            }
            &['\\', '$', ..] => {
                buf.extend("$".chars());
                s = &s[2..];
            }
            &[c, ..] => {
                buf.push(c);
                s = &s[1..];
            }
            &[] => {
                if !buf.is_empty() {
                    tokens.push(Text(buf));
                }
                return tokens;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ParsedKatexable::*;
    use super::*;
    use rstest::rstest;

    mod token_test {
        use super::Token;
        use super::Token::*;
        use rstest::rstest;

        #[rstest]
        #[case(
            "foo $\\frac{1}{2}$",
            vec![Text("foo ".into()), SingleDollar, Text("\\frac{1}{2}".into()), SingleDollar]
        )]
        #[case(
            "foo $\\frac{1}{",
            vec![Text("foo ".into()), SingleDollar, Text("\\frac{1}{".into())]
        )]
        #[case(
            "foo $$\\frac{1}{",
            vec![Text("foo ".into()), DoubleDollar, Text("\\frac{1}{".into())]
        )]
        #[case(
            "foo $$\\frac{1}{2}$$ foo",
            vec![
                Text("foo ".into()),
                DoubleDollar,
                Text("\\frac{1}{2}".into()),
                DoubleDollar,
                Text(" foo".into())
            ]
        )]
        #[case(
            "foo \\$$\\frac{1}{",
            vec![Text("foo $$\\frac{1}{".into())]
        )]
        fn test_tokenize(#[case] input: &str, #[case] expected: Vec<Token>) {
            let chars: Vec<char> = input.chars().collect::<Vec<_>>();
            let actual = crate::transform::katex::tokenize(&chars);
            assert_eq!(actual, expected);
        }
    }

    #[rstest]
    #[case(
        "foo $\\frac{1}{2}$",
        vec![Text("foo ".into()), InlineMath("\\frac{1}{2}".into())]
    )]
    #[case(
        "foo $\\frac{1}{",
        vec![Text("foo $\\frac{1}{".into())]
    )]
    #[case(
        "foo $$\\frac{1}{",
        vec![Text("foo $$\\frac{1}{".into())]
    )]
    #[case(
        "foo $$\\frac{1}{2}$$ foo",
        vec![Text("foo ".into()), BlockMath("\\frac{1}{2}".into()), Text(" foo".into())]
    )]
    #[case(
        "foo \\$$\\frac{1}{",
        vec![Text("foo $$\\frac{1}{".into())]
    )]
    fn test_find_katex(#[case] input: &str, #[case] expected: Vec<ParsedKatexable>) {
        let actual = find_katex(input);
        assert_eq!(actual, expected);
    }
}
