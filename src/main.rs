use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::process::{exit, Command, Stdio};

fn main() -> anyhow::Result<()> {
  let re = Regex::new(r#"(.+):\s*(\d{1,3})%\s*\((\d+)/(\d+)\)(?:,\s*(.*?)\s*\|\s*(.*/s))?\s*"#)?;
  let git_url = "https://github.com/Orange-OpenSource/hurl";

  let mut git = Command::new("git")
    .args(["clone", "--progress", git_url])
    .env("LANG", "C")
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .stdin(Stdio::null())
    .spawn()?;

  let bar = ProgressBar::new(1);
  bar.set_style(
    ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
      .unwrap()
      .progress_chars("=>-"),
  );

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
      // print!("{}, {}%, {}/{}", &cap[1], &cap[2], &cap[3], &cap[4]);
      // if let Some(cap5) = cap.get(5) {
      //   println!(", {}, {}", cap5.as_str(), &cap[6]);
      // } else {
      //   println!();
      // }

      let msg = &cap[1];
      let pos: u64 = cap[3].parse()?;
      let len: u64 = cap[4].parse()?;

      bar.set_position(pos);
      if bar.message() != msg {
        bar.set_message(msg.to_string());
      }
      if bar.length() != Some(len) {
        bar.set_length(len);
      }
    }
    stderr_string += &s;
    buf.clear();
  }

  let output = git.wait_with_output()?;
  if !output.status.success() {
    eprintln!("Child failed with {}", output.status);
    eprint!("{stderr_string}");
    exit(1);
  }

  Ok(())
}
