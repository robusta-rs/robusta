#![expect(clippy::print_stdout, clippy::print_stderr)]

use std::{
    env,
    fmt::Write as _,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::ExitCode,
    thread,
    time::{Duration, Instant},
};

/// Found in the working directory or its nearest ancestor that has one.
const BEHAVIOR_FILE: &str = "robusta-fake.properties";

/// What one run does: the behavior file's `record`, `exit`, `activity`, and
/// `await-release` lines, overridden by `--record=`, `--exit=`, `--activity=`,
/// and `--await-release` arguments.
#[derive(Default)]
struct Behavior {
    record: Option<PathBuf>,
    exit: u8,
    activity: Option<PathBuf>,
    await_release: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(exit) => ExitCode::from(exit),
        Err(error) => {
            eprintln!("fake robusta: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> io::Result<u8> {
    let arguments: Vec<String> =
        env::args_os().skip(1).map(|argument| argument.to_string_lossy().into_owned()).collect();
    let cwd = env::current_dir()?;
    let behavior = Behavior::read(&cwd, &arguments)?;

    if let Some(activity) = &behavior.activity {
        record_activity(activity, "enter", 1)?;
        if behavior.await_release {
            await_release(activity)?;
        } else {
            thread::sleep(Duration::from_millis(200));
        }
    }

    if let Some(record) = &behavior.record {
        let mut lines = format!("cwd={}\n", cwd.display());
        for (index, argument) in arguments.iter().enumerate() {
            let _ = writeln!(lines, "arg[{index}]={argument}");
        }
        OpenOptions::new().create(true).append(true).open(record)?.write_all(lines.as_bytes())?;
    }
    println!("fake robusta stdout");
    eprintln!("fake robusta stderr");

    if let Some(activity) = &behavior.activity {
        record_activity(activity, "exit", -1)?;
    }
    Ok(behavior.exit)
}

impl Behavior {
    fn read(cwd: &Path, arguments: &[String]) -> io::Result<Self> {
        let mut behavior = Self::default();
        if let Some(file) = cwd.ancestors().map(|dir| dir.join(BEHAVIOR_FILE)).find(|f| f.is_file())
        {
            for line in fs::read_to_string(file)?.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    behavior.set(key.trim(), value.trim())?;
                }
            }
        }
        for argument in arguments {
            if argument == "--await-release" {
                behavior.await_release = true;
            } else if let Some((key, value)) =
                argument.strip_prefix("--").and_then(|option| option.split_once('='))
                && matches!(key, "record" | "exit" | "activity")
            {
                behavior.set(key, value)?;
            }
        }
        Ok(behavior)
    }

    fn set(&mut self, key: &str, value: &str) -> io::Result<()> {
        match key {
            "record" => self.record = Some(PathBuf::from(value)),
            "exit" => {
                self.exit = value.parse().map_err(|_| invalid(&format!("exit code {value:?}")))?;
            }
            "activity" => self.activity = Some(PathBuf::from(value)),
            "await-release" => self.await_release = value == "true",
            _ => return Err(invalid(&format!("behavior key {key:?}"))),
        }
        Ok(())
    }
}

/// Appends `event pid active` to `events` and keeps the count of running
/// fakes in `active`, under a lock shared by every fake using `directory`.
fn record_activity(directory: &Path, event: &str, delta: i32) -> io::Result<()> {
    let lock =
        OpenOptions::new().write(true).create(true).truncate(false).open(directory.join("lock"))?;
    lock.lock()?;
    let counter = directory.join("active");
    let active = match fs::read_to_string(&counter) {
        Ok(text) => text.trim().parse::<i32>().map_err(|_| invalid(&format!("count {text:?}")))?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => 0,
        Err(error) => return Err(error),
    } + delta;
    fs::write(&counter, active.to_string())?;
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(directory.join("events"))?
        .write_all(format!("{event} {} {active}\n", std::process::id()).as_bytes())
}

fn await_release(directory: &Path) -> io::Result<()> {
    fs::write(directory.join("waiting"), "")?;
    let deadline = Instant::now() + Duration::from_secs(20);
    while !directory.join("release").exists() {
        if Instant::now() > deadline {
            return Err(io::Error::other("unrelated build work did not release the process"));
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

fn invalid(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("invalid {what}"))
}
