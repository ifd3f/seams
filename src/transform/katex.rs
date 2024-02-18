use std::{
    error::Error,
    fmt::Display,
    process::{ExitStatus, Stdio},
    str::Utf8Error,
};

use comrak::nodes::NodeValue;
use futures::{stream::FuturesOrdered, StreamExt};
use tokio::{io::AsyncWriteExt, process::Command, time::Instant};
use tracing::trace;

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

#[derive(Clone, Copy, Debug, Default)]
pub enum MathMode {
    #[default]
    Inline,
    Display,
}

impl MathMode {
    pub fn html_tag_name(&self) -> &str {
        match self {
            MathMode::Inline => "m",
            MathMode::Display => "M",
        }
    }

    pub fn opening_tag(&self) -> &str {
        match self {
            MathMode::Inline => "<m>",
            MathMode::Display => "<M>",
        }
    }

    pub fn closing_tag(&self) -> &str {
        match self {
            MathMode::Inline => "</m>",
            MathMode::Display => "</M>",
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn transform_math(source: &str, display_mode: MathMode) -> Result<String, KatexError> {
    match transform_math_raw(source, display_mode).await {
        Ok(r) => Ok(r),
        Err(e) => Err(KatexError {
            source: source.to_owned(),
            kind: e,
        }),
    }
}

#[tracing::instrument(skip_all)]
async fn transform_math_raw(source: &str, display_mode: MathMode) -> Result<String, KatexErrorKind> {
    let mut cmd = Command::new("katex");
    cmd.arg("--trust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    if let MathMode::Display = display_mode {
        cmd.arg("--display-mode");
    }

    trace!(?cmd, ?source, "executing katex");

    let start = Instant::now();
    let mut proc = cmd.spawn()?;
    trace!(?source, "writing source code");
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
