#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<() , Box<dyn std:: error::Error>>  {
    // let args = Cli::parse();

    let content  = std::fs::read_to_string("test.txt")?;
    println!("file content {}:" , content);
    Ok(())

 

    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Cant deal with {}, just exit here", error); }
    // };

    // println!("file content: {}", content);

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("matched line: {}", line);
    //     }
    // }
}
