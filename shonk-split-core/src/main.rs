use shonk_split_core::extract_text_from_pdf_with_qpdf;
use shonk_split_core::extract::rewe::extract_positions_from_rewe_text;

fn main() {
    let pdf_path = "../samples/REWE-eBon.pdf";

    let extracted_text = extract_text_from_pdf_with_qpdf(pdf_path).unwrap();
    let extracted_positions = extract_positions_from_rewe_text(extracted_text.as_str());
    println!("Positions: {extracted_positions:#?}");
}
