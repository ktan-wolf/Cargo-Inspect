use walkdir::WalkDir;
use std::fs;

pub fn run(){
    println!("🧠 Scanning for unsafe code...");

    let mut count = 0;

    for entry in WalkDir::new("./src"){
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s| s.to_str()) == Some("rs"){
            let code = fs::read_to_string(entry.path()).unwrap_or_default();
            if code.contains("unsafe"){
                println!("⚠️ Found in : {}" , entry.path().display());
                count += 1;
            }
        }
    }

    if count == 0 {
        println!("✅ No unsafe code found!");
    } else {
        println!("🚨 Total unsafe files: {}", count);
    }
}
