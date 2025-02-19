use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rand::Rng;
use serde::Serialize;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use std::error::Error;
use actix_cors::Cors;
use actix_web::middleware::Logger;

//use std::env;

#[derive(Serialize, Clone)]
struct Bird {
    name: String,
    image: String,
    description: String,
}

fn get_random_row(file_path: &str, _seed: usize) -> Result<Bird, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    let mut records: Vec<Bird> = Vec::new();

    // Read all records into memory
    for result in rdr.records() {
        let record = result?;
        let bird = Bird {
            name: record[0].to_string(),
            image: record[1].to_string(), // filepath
            description: record[2].to_string(),
        };
        records.push(bird);
    }

    // Check if there are any birds to choose from
    if records.is_empty() {
        return Err("No birds found in the file.".into());
    }

    let mut rng = rand::thread_rng();
    let random_bird = records.choose(&mut rng).ok_or("Failed to choose a random bird")?;

    Ok(random_bird.clone()) // Clone the selected bird and return it
}

async fn bird() -> impl Responder {
    // Generate a random seed for the bird selection
    let seed: u32 = rand::thread_rng().gen_range(2..=50);
    //println!("Current directory: {:?}", env::current_dir());
    // Get a random bird from CSV
    match get_random_row("./birds.csv", seed as usize) {
        
        Ok(random_bird) => HttpResponse::Ok().json(random_bird), // Retuxrn the bird as JSON
        Err(e) => HttpResponse::InternalServerError().body(format!("Error retrieving bird data: {}", e)),
    }
}

async fn generate_random_number() -> impl Responder {
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    format!("Random number: {}", random_number)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())  // Optional: for logging requests
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["content-type"])
                    .max_age(3600),
            )
            .route("/random", web::get().to(generate_random_number)) // test endpoint
            .route("/bird", web::get().to(bird)) // endpoint to get random bird
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

