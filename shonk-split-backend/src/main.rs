use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use shonk_split_core::extract::rewe::extract_positions_from_rewe_text;
use shonk_split_core::extract_text_from_pdf_with_qpdf;

#[get("/rewe_data")]
async fn rewe_data() -> impl Responder {
    let pdf_path = "../samples/REWE-eBon.pdf";

    let extracted_text = extract_text_from_pdf_with_qpdf(pdf_path).unwrap();
    let extracted_positions = extract_positions_from_rewe_text(extracted_text.as_str());

    HttpResponse::Ok().json(extracted_positions)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(rewe_data)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
