//! Compatibility launcher for the old `obscura-worker` entry point.
//!
//! The engine used to ship a second, full copy of the binary so `scrape`
//! could spawn workers. The worker now lives behind the hidden
//! `obscura __worker` subcommand; this launcher keeps the documented
//! `obscura-worker` path working at a fraction of the size.
//!
//! It only exists for users and scripts that invoke `obscura-worker`
//! directly. `scrape` itself re-execs `obscura __worker` and never uses this
//! file.

use std::process::ExitCode;

fn main() -> ExitCode {
    let exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("obscura-worker: cannot resolve current executable: {error}");
            return ExitCode::FAILURE;
        }
    };

    let sibling = exe.with_file_name(if cfg!(windows) {
        "obscura.exe"
    } else {
        "obscura"
    });

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;

        // `exec` replaces this process, so stdin/stdout stay wired exactly as
        // the caller set them up and no extra process is left behind.
        let error = std::process::Command::new(&sibling)
            .arg("__worker")
            .args(std::env::args_os().skip(1))
            .exec();
        eprintln!(
            "obscura-worker: cannot exec {}: {error}",
            sibling.display()
        );
        ExitCode::FAILURE
    }

    #[cfg(not(unix))]
    {
        match std::process::Command::new(&sibling)
            .arg("__worker")
            .args(std::env::args_os().skip(1))
            .status()
        {
            Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
            Err(error) => {
                eprintln!(
                    "obscura-worker: cannot run {}: {error}",
                    sibling.display()
                );
                ExitCode::FAILURE
            }
        }
    }
}
