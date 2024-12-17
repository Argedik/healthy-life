use std::fs;
use std::io::{self, Write};
use std::path::Path;

// fn main() {
//     match embed_content("../header/basic/index.html") {
//         Ok(_) => println!("Content embedded successfully!"),
//         Err(e) => eprintln!("Error: {:?}", e),
//     }
// }

fn embed_content(file_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    inject_html("embeddedContent", &content)?;
    
    add_stylesheet("../header/basic/css/style.css")?;
    add_script("../header/basic/js/index.js")?;
    Ok(())
}

fn inject_html(element_id: &str, content: &str) -> io::Result<()> {
    println!("Injecting HTML to element with id '{}':\n{}", element_id, content);
    Ok(())
}

fn add_stylesheet(file_path: &str) -> io::Result<()> {
    if Path::new(file_path).exists() {
        println!("Linking stylesheet: {}", file_path);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Stylesheet not found"))
    }
}

fn add_script(file_path: &str) -> io::Result<()> {
    if Path::new(file_path).exists() {
        println!("Linking script: {}", file_path);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Script not found"))
    }
}
