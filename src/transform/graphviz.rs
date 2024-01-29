use std::{
    process::{ExitStatus, Stdio},
    str::Utf8Error,
};

use tokio::{io::AsyncWriteExt, process::Command};
use tracing::trace;

#[derive(thiserror::Error, Debug)]
pub enum GraphvizError {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Error parsing UTF-8 from CLI output: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("Executing dot failed with status code {0}. stderr:\n{1}")]
    CmdFailed(ExitStatus, String),
}

#[tracing::instrument]
pub async fn transform_graphviz(source: &str) -> Result<String, GraphvizError> {
    let mut cmd = Command::new("dot");
    cmd.arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    trace!(?cmd, "executing dot");

    let mut proc = cmd.spawn()?;
    trace!(%source, "writing source code");
    let mut stdin = proc.stdin.take().unwrap();
    stdin.write(source.as_bytes()).await?;

    let result = proc.wait_with_output().await?;

    let status = result.status;
    trace!(?status, "command exited");

    let svg = String::from_utf8(result.stdout).unwrap();
    let log = String::from_utf8_lossy(&result.stderr);

    if !status.success() {
        return Err(GraphvizError::CmdFailed(status, log.into()));
    }

    Ok(svg)
}
