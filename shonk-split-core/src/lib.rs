pub mod extract;

use std::error::Error;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

/// Extracts text from a PDF, decrypting it with qpdf if needed.
///
/// Automatically uses a temporary file for the decrypted version.
pub fn extract_text_from_pdf_with_qpdf<P: AsRef<Path>>(
    input_pdf: P,
) -> Result<String, Box<dyn Error>> {
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path();

    let status = Command::new("qpdf")
        .args([
            "--decrypt",
            input_pdf.as_ref().to_str().unwrap(),
            temp_path.to_str().unwrap(),
        ])
        .status()?;

    if !status.success() {
        return Err("qpdf decryption failed".into());
    }

    let text = pdf_extract::extract_text(temp_path)?;

    Ok(text)
}
