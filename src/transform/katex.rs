use std::{
    process::{ExitStatus, Stdio},
    str::Utf8Error,
};

use itertools::Itertools;
use nom::FindToken;
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
pub enum Block {
    Plain(String),
    BlockMath(String),
    InlineMath(String),
}

pub fn find_katex(s: &str) -> Vec<Block> {
    use Block::*;
    let chars = s.chars().collect::<Vec<char>>();
    let mut s = &chars[..];
    let mut tokens = vec![];
    let mut buf = String::new();

    macro_rules! handle_delimiter {
        ($end_delim:expr, $constructor:expr) => {
            match find_end_of_block($end_delim, &s[$end_delim.len()..]) {
                Some((block, rest)) => {
                    if !buf.is_empty() {
                        tokens.push(Plain(buf));
                        buf = String::new();
                    }
                    tokens.push($constructor(block.iter().collect()));
                    s = rest;
                }
                None => {
                    // We encountered end of string before finding the end,
                    // treat this as part of the outer text.
                    buf.extend(s);
                    if !buf.is_empty() {
                        tokens.push(Plain(buf));
                    }
                    return tokens;
                }
            }
        };
    }

    loop {
        // Invariant: outside of the match, s is inside Plain

        match s {
            &['$', '$', ..] => handle_delimiter!(&['$', '$'], BlockMath),
            &['$', ..] => handle_delimiter!(&['$'], InlineMath),
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
                    tokens.push(Plain(buf));
                }
                return tokens;
            }
        }
    }
}

/// Returns (text before delim, text after delim)
fn find_end_of_block<'a>(
    end_delim: &'a [char],
    text: &'a [char],
) -> Option<(&'a [char], &'a [char])> {
    let mut i = 0usize;
    while i < text.len() {
        if text[i..].starts_with(&['\\']) {
            // If we find a backslash, treat it as part of the text,
            // whether or not it is followed by end_delim.
            //
            // Examples of fully-formed math blocks:
            // $ \frac{1}{2} $
            // $ \$1 $
            // $ \$$
            i += 1 + end_delim.len();
            continue;
        } else if text[i..].starts_with(end_delim) {
            // Found the delimiter
            return Some((&text[..i], &text[i + end_delim.len()..]));
        } else {
            i += 1;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::Block::*;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "foo $\\frac{1}{2}$",
        vec![
            Plain("foo ".into()),
            InlineMath("\\frac{1}{2}".into())
        ]
    )]
    #[case(
        "foo $\\frac{1}{",
        vec![Plain("foo $\\frac{1}{".into())]
    )]
    #[case(
        "foo $$\\frac{1}{",
        vec![Plain("foo $$\\frac{1}{".into())]
    )]
    #[case(
        "foo $$\\frac{1}{2}$$ foo",
        vec![
            Plain("foo ".into()),
            BlockMath("\\frac{1}{2}".into()),
            Plain(" foo".into())
        ]
    )]
    #[case(
        "foo \\$$\\frac{1}{",
        vec![Plain("foo $$\\frac{1}{".into())]
    )]
    #[case(
        "i got $10 in my bank account",
        vec![
            Plain("i got $10 in my bank account".into()),
        ]
    )]
    #[case(
        "i got \\$10 in my bank account and $ \\$20 $ in my pocket",
        vec![
            Plain("i got $10 in my bank account and ".into()),
            InlineMath(" \\$20 ".into()),
            Plain(" in my pocket".into()),
        ]
    )]
    #[case(
        "$$ $ $$",
        vec![
            BlockMath(" $ ".into()),
        ]
    )]
    #[case(
        "$ $$$$$",
        vec![
            InlineMath(" ".into()),
            BlockMath("".into()),
        ]
    )]
    #[case(
        "$ $$$",
        vec![
            InlineMath(" ".into()),
            Plain("$$".into()),
        ]
    )]
    fn test_find_katex(#[case] input: &str, #[case] expected: Vec<Block>) {
        let actual = find_katex(input);
        assert_eq!(actual, expected);
    }
}
