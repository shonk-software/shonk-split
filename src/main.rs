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
        .args(&[
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

/// Extracts the lines containing positions from the REWE receipt text.
pub fn rewe_extract_positions_lines(text: &str) -> Vec<&str> {
    let lines = text.lines().into_iter();

    let lines = lines.skip_while(|line| line.trim() != "EUR").skip(1);

    let positions = lines.take_while(|line| line.trim() != "--------------------------------------").collect();
    positions
}

struct Position {
    amount: u32,
    name: String,
    price: f32,
}

pub fn rewe_extract_positions(lines: Vec<&str>) -> Vec<Position> {
    todo!()
}

fn main() {
    let pdf_path = "samples/REWE-eBon.pdf";

    let extracted_text = extract_text_from_pdf_with_qpdf(pdf_path).unwrap();
    //println!("{}", extracted_text);
    let positions = rewe_extract_positions_lines(&extracted_text);
    println!("{:#?}", positions);

}
