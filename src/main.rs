use std::process::Command;

use pdf_extract::extract_text;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "samples/REWE-eBon.pdf";

    // Create a temporary file for decrypted PDF
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap();

    // Use qpdf to decrypt the file into the temp file
    let status = Command::new("qpdf")
        .args(&["--decrypt", input_path, temp_path])
        .status()?;

    if !status.success() {
        return Err("qpdf decryption failed".into());
    }

    // Use pdf_extract to extract text
    let text = extract_text(temp_path)?;
    println!("Extracted Text:\n{}", text);

    Ok(())
}
