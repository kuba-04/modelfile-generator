use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> io::Result<()> {
    // Get directory path from user
    let dir_path = get_user_input("Enter the directory path where your GGUF file is located: ")?;
    let dir_path = Path::new(&dir_path);

    if !dir_path.exists() || !dir_path.is_dir() {
        eprintln!(
            "Error: Directory does not exist or is not a directory: {}",
            dir_path.display()
        );
        std::process::exit(1);
    }

    // Get GGUF file name from user
    let gguf_filename = get_user_input("Enter the GGUF file name (including extension): ")?;
    let gguf_path = dir_path.join(&gguf_filename);

    if !gguf_path.exists() {
        eprintln!("Error: GGUF file does not exist: {}", gguf_path.display());
        std::process::exit(1);
    }

    // Create the output Modelfile path in the same directory
    let modelfile_path: PathBuf = dir_path.join("Modelfile");

    // Create the Modelfile content with proper template formatting
    let modelfile_content = format!(
        r#"FROM "{}"
PARAMETER stop "<|im_start|>"
PARAMETER stop "<|im_end|>"
TEMPLATE """
<|im_start|>system
{{{{ .System }}}}<|im_end|>
<|im_start|>user
{{{{ .Prompt }}}}<|im_end|>
<|im_start|>assistant
"""
"#,
        gguf_path.display()
    );

    // Write the content to the Modelfile
    let mut file = File::create(&modelfile_path)?;
    file.write_all(modelfile_content.as_bytes())?;

    println!(
        "\n🍻 Successfully created Modelfile at: {}",
        modelfile_path.display()
    );
    Ok(())
}
