use std::{error::Error, fs::File};

fn getenv(name: &str) -> String {
	return std::env::var(name).unwrap_or_default();
}

/// コマンドを実行する
fn execute_command(command: &[&str]) -> Result<(), Box<dyn Error>> {
	if cfg!(target_os = "windows") {
		println!("CMD> [{}]", &command.join(" "));

		let mut process = std::process::Command::new("cmd").arg("/C").args(command).spawn()?;
		let status = process.wait()?;
		if !status.success() {
			let exit_code = status.code().unwrap();
			println!("[ERROR] command exited with status: {}", exit_code);
			std::process::exit(1);
		}
	} else {
		println!("SH> [{}]", &command.join(" "));

		let mut output = std::process::Command::new(command[0]).args(&command[1..]).spawn()?;
		// let mut output = std::process::Command::new("sh").arg("-c").args(command).spawn()?;
		let status = output.wait()?;
		if !status.success() {
			let exit_code = status.code().unwrap();
			println!("[ERROR] command exited with status: {}", exit_code);
			std::process::exit(1);
		}
	}

	return Ok(());
}

/// テキストファイルを作成する
fn create_text_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
	use std::io::Write;

	let mut file = File::create(path)?;
	file.write_all(content.as_bytes())?;

	return Ok(());
}

/// タイムスタンプ文字列を返します。
///
/// # Returns
/// タイムスタンプ
pub fn get_current_timestamp() -> String {
	let date = chrono::Local::now();
	let t = date.format("%Y-%m-%d %H:%M:%S%.3f");
	return format!("{}", t);
}

/// タイムスタンプ文字列を返します。
///
/// # Returns
/// タイムスタンプ
pub fn get_current_timestamp2() -> String {
	let date = chrono::Local::now();
	let t = date.format("%Y%m%d-%H%M%S");
	return format!("{}", t);
}

/// ブランチを作成します。
fn make_branch() -> Result<(), Box<dyn Error>> {
	let timestamp = get_current_timestamp2();

	println!("[INFO] Making branch...");
	let branch_name = format!("feature/new-feature-{}", &timestamp);
	execute_command(&["git", "checkout", "-b", &branch_name])?;
	execute_command(&["git", "push", "--set-upstream", "origin", &branch_name])?;

	println!("[INFO] Modifying a file...");
	create_text_file("timestamp.tmp", &timestamp)?;

	println!("[INFO] Commiting a file...");

	execute_command(&["git", "add", "timestamp.tmp"])?;

	execute_command(&["git", "commit", "-m", "wip"])?;

	execute_command(&["git", "push"])?;

	execute_command(&["git", "checkout", "main"])?;

	return Ok(());
}

/// 変更のあったファイルを現在のブランチに追加して push します。
fn execute_when_push() -> Result<(), Box<dyn Error>> {
	let github_token = getenv("GITHUB_TOKEN");
	println!("[DEBUG] token: {}", github_token);

    let github_repository = getenv("GITHUB_REPOSITORY");
	println!("[DEBUG] github_repository: {}", github_repository);

    let github_actor = getenv("GITHUB_ACTOR");
	println!("[DEBUG] github_actor: {}", github_actor);

	let url = format!(
		"https://github-actions:{GITHUB_TOKEN}@github.com/{GITHUB_REPOSITORY}",
		GITHUB_TOKEN = github_token,
		GITHUB_REPOSITORY = github_repository
	);
	println!("[DEBUG] url: {}", url);
	execute_command(&["git", "remote", "set-url", "origin", &url])?;

	execute_command(&["git", "config", "--global", "user.name", &github_actor])?;

	let mail = format!("{GITHUB_ACTOR}@users.noreply.github.com", GITHUB_ACTOR = github_actor);
	execute_command(&["git", "config", "--global", "user.email", &mail])?;

	execute_command(&["git", "add", "."])?;

	execute_command(&["git", "commit", "-m", "add tmp"])?;

	execute_command(&["git", "push"])?;

	return Ok(());
}

fn execute_test() -> Result<(), Box<dyn Error>> {
	execute_command(&["git", "checkout", "main"])?;
	return Ok(());
}

/// 要求に応じたバッチ処理を実行します。
fn execute(request: &str) -> Result<(), Box<dyn Error>> {
	if request == "1" {
		return make_branch();
	}
	if request == "--push" {
		return execute_when_push();
	}
	if request == "--test" {
		return execute_test();
	}

	panic!("ERROR: Invalid request: {}", request);
}

/// アプリケーションのエントリポイントです。
fn main() {
	let args = std::env::args().skip(1).collect::<Vec<_>>();
	let request = if 0 < args.len() { &args[0] } else { "" };
	let result = execute(request);
	if let Err(e) = result {
		println!("[ERROR] {}", e);
		return;
	}

	println!("Ok.");
}
