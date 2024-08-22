mod routes; // Declare the routes module
mod models; // Declare the models module

use warp::Filter;
use handlebars::Handlebars;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize Handlebars template engine
    let handlebars = Arc::new(Handlebars::new());

    // Load the routes from the `routes` module
    let routes = routes::index()
        .or(routes::submit(handlebars.clone()));
     let port=8000;
    // Start the Warp server on localhost:8000
    println!("Starting server on http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

}
