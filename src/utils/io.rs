use std::io::{self, StderrLock, StdoutLock, Write, stderr, stdout};

use crate::utils::logger;

type StdioOutputLocks = (StdoutLock<'static>, StderrLock<'static>);

pub fn lock_and_flush_output_stdio() -> io::Result<StdioOutputLocks> {
    logger::flush_messages();

    let mut stdout = stdout().lock();
    stdout.flush()?;
    let mut stderr = stderr().lock();
    stderr.flush()?;

    Ok((stdout, stderr))
}
