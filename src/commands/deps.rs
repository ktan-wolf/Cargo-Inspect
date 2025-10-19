use cargo_metadata::MetadataCommand;
use colored::*;

pub fn run(){

    println!("{}", "🔍 Inspecting dependencies...".bold().cyan());

    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to read Cargo Metadata");

    for package in metadata.packages {
        println!("{} {}" , 
            package.name.green().bold(),
            package.version.to_string().yellow()
        );
    }
    println!("{}" ,"✅ Done!".bold().green());
}
