use regex::Regex;
use std::io::{BufRead, BufReader};
use std::process::{exit, Command, Stdio};

fn main() -> anyhow::Result<()> {
  let re = Regex::new(r#"(.+):\s*(\d{1,3})%\s*\((\d+)/(\d+)\)(?:,\s*(.*?)\s*\|\s*(.*/s))?\s*"#)?;

  let mut git = Command::new("git")
    .args([
      "clone",
      "--progress",
      "https://git.yuuta.moe/Dress.git",
    ])
    .env("LANG", "C")
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .stdin(Stdio::null())
    .spawn()?;

  let mut stderr_string = String::new();
  let mut stderr = BufReader::new(git.stderr.take().unwrap());
  let mut buf = Vec::new();
  loop {
    let len = stderr.read_until(b'\r', &mut buf)?;
    if len == 0 {
      break;
    }
    let s = String::from_utf8_lossy(&buf[..len]);
    if let Some(cap) = re.captures(&s) {
      print!("{}, {}%, {}/{}", &cap[1], &cap[2], &cap[3], &cap[4]);
      if let Some(cap5) = cap.get(5) {
        println!(", {}, {}", cap5.as_str(), &cap[6]);
      } else {
        println!();
      }
    }
    stderr_string += &s;
    buf.clear();
  }

  let output = git.wait_with_output()?;
  if !output.status.success() {
    eprintln!("Child failed with {}", output.status);
    eprintln!("{stderr_string}");
    exit(1);
  }

  Ok(())
}
