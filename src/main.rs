use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::sync::Mutex;
use tera::{Context, Tera};
mod conversion;

// State to hold Tera templates and conversion history
struct AppState {
    tera: Tera,
    history: Mutex<Vec<String>>, // Store conversion history
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Tera templates
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Template parsing error: {}", e);
            println!("Ensure the 'templates' directory exists and contains 'index.html'.");
            std::process::exit(1);
        }
    };

    // Debug: Check if the template is loaded
    if !tera.get_template_names().any(|name| name == "index.html") {
        println!("Error: 'index.html' not found in the 'templates' directory.");
        std::process::exit(1);
    }

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                tera: tera.clone(),
                history: Mutex::new(Vec::new()),
            }))
            .route("/", web::get().to(index))
            .route("/convert", web::post().to(convert))
            .route("/history", web::get().to(history))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Handler for the home page
async fn index(data: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    let ctx = Context::new();
    match data.tera.render("index.html", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

// Handler for the conversion API
async fn convert(data: web::Data<AppState>, form: web::Form<ConversionForm>) -> HttpResponse {
    let mut ctx = Context::new();
    let result = match form.conversion_type.as_str() {
        "f_to_c" => {
            let celsius = conversion::fahrenheit_to_celsius(form.temperature);
            let conversion_text = format!("{}°F -> {:.2}°C", form.temperature, celsius);
            data.history.lock().unwrap().push(conversion_text.clone());
            conversion_text
        }
        "c_to_f" => {
            let fahrenheit = conversion::celsius_to_fahrenheit(form.temperature);
            let conversion_text = format!("{}°C -> {:.2}°F", form.temperature, fahrenheit);
            data.history.lock().unwrap().push(conversion_text.clone());
            conversion_text
        }
        "k_to_c" => {
            let celsius = conversion::kelvin_to_celsius(form.temperature);
            let conversion_text = format!("{}K -> {:.2}°C", form.temperature, celsius);
            data.history.lock().unwrap().push(conversion_text.clone());
            conversion_text
        }
        "c_to_k" => {
            let kelvin = conversion::celsius_to_kelvin(form.temperature);
            let conversion_text = format!("{}°C -> {:.2}K", form.temperature, kelvin);
            data.history.lock().unwrap().push(conversion_text.clone());
            conversion_text
        }
        _ => "Invalid conversion type".to_string(),
    };

    ctx.insert("result", &result);
    match data.tera.render("index.html", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

// Handler for the history page
async fn history(data: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    let mut ctx = Context::new();
    let history = data.history.lock().unwrap().clone();
    ctx.insert("history", &history);
    match data.tera.render("history.html", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

// Form data structure
#[derive(serde::Deserialize)]
struct ConversionForm {
    conversion_type: String,
    temperature: f64,
}
