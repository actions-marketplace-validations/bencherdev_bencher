use std::convert::TryFrom;

#[derive(Debug)]
pub struct Output {
    pub status: std::process::ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl TryFrom<std::process::Output> for Output {
    type Error = std::string::FromUtf8Error;

    fn try_from(output: std::process::Output) -> Result<Self, Self::Error> {
        Ok(Output {
            status: output.status,
            stdout: String::from_utf8(output.stdout)?,
            stderr: String::from_utf8(output.stderr)?,
        })
    }
}
