use std::{fs::File, error::Error};

fn spawn_os_command(command: &str) -> Result<(), Box<dyn Error>> {
    let output = std::process::Command::new("cmd")
        .args(&["/C", command])
        .output()?;

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    return Ok(());
}

fn create_text_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    use std::io::Write;

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    return Ok(());
}

fn get_current_timestamp() -> String {
    use std::time::SystemTime;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");

    return format!("{}", timestamp.as_secs());
}

fn make_branch() -> Result<(), Box<dyn Error>> {
    let timestamp = get_current_timestamp();

    {
        println!("[INFO] Making branch...");
        let command = format!("git checkout -b feature/new-feature-{}", &timestamp);
        spawn_os_command(&command)?;
        spawn_os_command("git push --set-upstream origin develop")?;
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
    }

    return Ok(());
}

fn execute(request: &str) -> Result<(), Box<dyn Error>> {

    make_branch()?;

    // let mut response = String::new();
    // let mut file = File::open(request)?;
    // file.read_to_string(&mut response)?;
    // println!("{}", response);

    return Ok(());
}

fn main() {

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 0 {
        return;
    }
    let request = &args[0];
    let result = execute(&request);
    if let Err(e) = result {
        println!("{}", e);
        return;
    }
    println!("Hello, world!");
}
