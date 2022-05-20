use std::{error::Error, fs::File};

/// コマンドを実行する
fn spawn_os_command(command: &str) -> Result<(), Box<dyn Error>> {
    println!("[DEBUG] コマンドを実行中... {}", command);

    let output = std::process::Command::new("cmd")
        .args(&["/C", command])
        .output()?;

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

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

    {
        println!("[INFO] Making branch...");

        let command = format!("git checkout -b feature/new-feature-{}", &timestamp);
        spawn_os_command(&command)?;

        let command = format!(
            "git push --set-upstream origin feature/new-feature-{}",
            &timestamp
        );
        spawn_os_command(&command)?;
    }

    {
        println!("[INFO] Modifying a file...");
        create_text_file("timestamp.tmp", &timestamp)?;
    }

    {
        println!("[INFO] Commiting a file...");

        spawn_os_command("git add timestamp.tmp")?;

        spawn_os_command("git commit -m wip")?;

        spawn_os_command("git push")?;

        spawn_os_command("git checkout main")?;
    }

    return Ok(());
}

/// 要求に応じたバッチ処理を実行します。
fn execute(request: &str) -> Result<(), Box<dyn Error>> {
    if request == "1" {
        return make_branch();
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
    println!("Hello, world!");
}
