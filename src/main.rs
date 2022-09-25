//use ferris_says::say; // from the previous step
//use std::io::{stdout, BufWriter};
use std::fs;
use std::process::Command;

fn main()  -> std::io::Result<()> {
    //let stdout = stdout();
    //let message = String::from("Hello fellow Rustaceans!");
    //let width = message.chars().count();

    //let mut writer = BufWriter::new(stdout.lock());
    //say(message.as_bytes(), width, &mut writer).unwrap();
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    let hello = output.stdout;
    let hello_str = String::from_utf8(hello).unwrap();
    println!("{}",&hello_str);

    fs::copy("/Users/pwwang72/rust-gdb-example/lol.txt", "/Users/pwwang72/rust-gdb-example/bar.txt")?;
    let file_path =  "/Users/pwwang72/rust-gdb-example/bar.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("{}", contents);
    let test_json = json::parse(&contents).unwrap();
    assert!(test_json["test"] == "test_value");
    assert!(test_json["test_int"] == 1);
    assert!(test_json.has_key("test_int"));
    println!("value read from json is: {}, {}", test_json["test"], test_json["test_int"]);
    Ok(())
}
