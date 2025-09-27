use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::ptr::dangling;
use regex::{Regex, RegexBuilder};
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

#[derive(Debug)]
struct Position {
    name: String,
    amount: u32,
    price: f32,
}

fn get_price_indexes(line: &str) -> Option<(usize, usize)> {
    let chars = line.char_indices().collect::<Vec<(usize, char)>>();

    // ------------------- Extracting the price
    let end_index = *chars.iter()
        .rev()
        .skip_while(|(_, c)| !c.is_digit(10))
        .next()
        .map(|(idx, _)| idx).unwrap();

    let start_index = chars.iter()
        .rev()
        .skip_while(|(_, c)| *c != ',')
        .skip_while(|(_, c)| !c.is_whitespace())
        .next()
        .map(|(idx, _)| idx + 1).unwrap(); // Currently pointing at the whitespace next to the number

    Some((start_index, end_index))
}

fn parse_position_line(line: &str) -> Option<Position> {
    let line = line.trim().to_owned();

    let (start_index, end_index) = get_price_indexes(line.as_str())?;

    let price = &mut line[start_index..=end_index].replace(',', ".");
    let price: f32 = price.parse().unwrap();

    // --------------------- Extracting the name of the product - shrimple now :shrimp:
    let name = *&line[0..start_index].trim();

    Some(Position {
        amount: 1,
        name: name.to_owned(),
        price,
    })
}

fn parse_amount(amount: &str) -> Option<u32> {
    let amount = amount.trim();
    let split = amount.split_once(' ').unwrap();
    let amount: u32 = split.0.parse().unwrap();

    Some(amount)
}

// Sample lines:
// "KREUZKUEMMEL GEM                 1,99 B",
// "RINDER FOND                      2,58 B",
// "             2 Stk x    1,29",
// "TOMATENMARK                      0,89 B",
// "KIDNEYBOHNEN                     0,79 B",
fn rewe_extract_positions(lines: Vec<&str>) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let mut lines_iter = lines.into_iter().peekable();

    let amount_pattern = Regex::new(r"^.*\d+\s+Stk\s+x\s+\d+,\d\d$").unwrap();
    while let Some(line) = lines_iter.next() {
        let line = line.trim();
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        if amount_pattern.is_match(line) && !positions.is_empty() {
            let new_amount = parse_amount(line).unwrap();
            positions.last_mut().unwrap().amount = new_amount;
            continue;
        }

        if let Some(position) = parse_position_line(line) {
            positions.push(position);
            continue;
        }

        panic!("Could not parse line {}", line);
    }

    positions
}

fn main() {
    let pdf_path = "samples/REWE-eBon.pdf";

    let extracted_text = extract_text_from_pdf_with_qpdf(pdf_path).unwrap();
    //println!("{}", extracted_text);
    let position_lines = rewe_extract_positions_lines(&extracted_text);
    println!("{:#?}", position_lines);

    let extracted_positions = rewe_extract_positions(position_lines);
    println!("Positions: {:#?}", extracted_positions);
}
