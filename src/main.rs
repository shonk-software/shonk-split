mod extract;

use std::error::Error;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;
use crate::extract::rewe::extract_positions_from_rewe_text;

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

#[derive(Debug)]
struct Position {
    name: String,
    amount: u32,
    price: f32,
}

fn main() {
    let pdf_path = "samples/REWE-eBon.pdf";

    let extracted_text = extract_text_from_pdf_with_qpdf(pdf_path).unwrap();
    let extracted_positions = extract_positions_from_rewe_text(extracted_text.as_str());
    println!("Positions: {extracted_positions:#?}");
}
