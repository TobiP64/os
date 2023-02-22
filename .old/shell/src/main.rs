#![feature(stdio_locked)]

use std::io::{BufRead, Write};

fn main() {
	let args = std::env::args().map(|v| v.to_string()).collect::<Vec<_>>();
	
	if args.contains(&"-c".to_string()) {
		return;
	}
	
	let mut buf    = String::new();
	let mut stdin  = std::io::stdin_locked();
	let mut stdout = std::io::stdout_locked();
	let mut prompt = prompt();
	
	loop {
		stdout.write_all(prompt.as_bytes());
		stdout.flush();
		buf.clear();
		if let Err(e) = stdin.read_line(&mut buf) {
			let _ = writeln!(stdout, "failed to read from stdin: {}", e);
			std::process::exit(100);
		}
		
		let (cmd, args) = buf.split_once(' ').unwrap_or((&buf, ""));
		match cmd {
			"cd"    => cmd_cd(args),
			"ls"    => cmd_ls(args),
			"mk"    => cmd_mk(args),
			"rm"    => cmd_rm(args),
			"mv"    => cmd_mv(args),
			"cp"    => cmd_cp(args),
			"cat"   => cmd_cat(args),
			"tee"   => cmd_tee(args),
			"ln"    => cmd_ln(args),
			"tc"    => cmd_tc(args),
			cmd     => {
				unimplemented!()
			}
		}
	}
}

fn prompt() -> String {
	format!(
		"[{}@{} {}]",
		std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
		std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
		std::env::current_dir().map_or_else(|_| "?".to_string(), |v| v.to_string_lossy().into_owned())
	)
}

fn cmd_cd(buf: &str) -> std::io::Result<()> {
	std::env::set_current_dir(buf)
}

fn cmd_ls(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_mk(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_rm(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_mv(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_cp(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_cat(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_tee(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_ln(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn cmd_tc(buf: &str) -> std::io::Result<()> {
	Ok(())
}

fn echo(buf: &str) -> std::io::Result<()> {
	Ok(())
}