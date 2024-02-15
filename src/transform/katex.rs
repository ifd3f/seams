use std::{
    error::Error,
    fmt::Display,
    process::{ExitStatus, Stdio},
    str::Utf8Error,
};

use futures::{stream::FuturesOrdered, StreamExt};
use tokio::{io::AsyncWriteExt, process::Command, time::Instant};
use tracing::trace;

use crate::errors::Errors;

#[derive(Debug)]
pub struct KatexError {
    pub source: String,
    pub kind: KatexErrorKind,
}

impl Display for KatexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error parsing {:?}:\n  {}", self.source, self.kind)
    }
}

impl Error for KatexError {}

#[derive(thiserror::Error, Debug)]
pub enum KatexErrorKind {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Error parsing UTF-8 from CLI output: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("Executing dot failed with status code {0}. stderr:\n{1}")]
    CmdFailed(ExitStatus, String),
}

pub async fn transform_katex_str(s: &str) -> Result<String, Errors<KatexError>> {
    let mut fu = find_katex(s)
        .into_iter()
        .map(|n| async move {
            match &n {
                Block::Plain(t) => Ok(t.to_owned()),
                Block::BlockMath(m) => transform_math(m, true).await,
                Block::InlineMath(m) => transform_math(m, false).await,
            }
            .map_err(|k| KatexError {
                source: n.inner().to_owned(),
                kind: k,
            })
        })
        .collect::<FuturesOrdered<_>>();

    let mut errs = Errors::new();
    let mut out = "".to_string();
    while let Some(s) = fu.next().await {
        match s {
            Ok(s) => out.extend(s.chars()),
            Err(e) => errs.push(e),
        }
    }

    errs.into_result()?;

    Ok(out)
}

#[tracing::instrument(skip_all)]
pub async fn transform_math(source: &str, display_mode: bool) -> Result<String, KatexErrorKind> {
    let mut cmd = Command::new("katex");
    cmd.arg("--trust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    if display_mode {
        cmd.arg("--display");
    }

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
        return Err(KatexErrorKind::CmdFailed(status, log.into()));
    }

    Ok(svg)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Plain(String),
    BlockMath(String),
    InlineMath(String),
}

impl Block {
    fn inner(&self) -> &str {
        match self {
            Block::Plain(s) => s,
            Block::BlockMath(s) => s,
            Block::InlineMath(s) => s,
        }
    }
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
