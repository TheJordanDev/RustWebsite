use std::{env, fs::File, sync::Arc, thread};
use tiny_http::{Request, Response, Server, StatusCode};
use ascii::AsciiString;
use dotenv::dotenv;
mod templating;
fn main() {

    dotenv().ok();
    let address = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());

    let server = Arc::new(Server::http(format!("{}:{}", address, port)).unwrap());
    println!("Server started on http://localhost:{}", port);

    let mut handlers = Vec::new();

    for thread_num in 0..std::thread::available_parallelism().unwrap().into() {
        println!("Starting thread {}", thread_num);

        let server = server.clone();
        handlers.push(thread::spawn(move || server_thread(server)));
    }

    for handler in handlers {
        handler.join().unwrap();
    }

}

fn server_thread(server: Arc<Server>) {
    for request in server.incoming_requests() {
        println!(
            "Received request! Method: {:?}, Url: {:?} ",
            request.method(),
            request.url()
        );

        match request.url() {
            "" | "/" => {
                let response = Response::from_string(templating::render_home());

                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf-8").unwrap(),
                });

                let _ = request.respond(response);
            }
            "/about" => {
                let response = Response::from_string(templating::render_about());

                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf-8").unwrap(),
                });

                let _ = request.respond(response);
            }
            url => {
                if let Some(resource) = find_resource(url) {
                    let _ = request.respond(resource);
                } else {
                    serve_404(request);
                }
            }
        }

    }
}

fn find_resource(url: &str) -> Option<Response<File>> {
    let path = url.split('?').next().unwrap_or(url);
    let path = format!("public{}", path);

    if let Ok(file) = File::open(&path) {
        let mut response = Response::from_file(file);

        if let Some(extension) = path.split('.').last() {
            let mime = match extension {
                "css" => "text/css",
                "html" => "text/html",
                "js" => "text/javascript",
                "png" => "image/png",
                "svg" => "image/svg+xml",
                "json" => "application/json",
                _ => "text/plain",
            };

            response = response.with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii(mime).unwrap(),
            });
        }

        Some(response)
    } else {
        None
    }
}

fn serve_404(request: Request) {
    let response = Response::from_string("404 Not Found");
    let response = response.with_status_code(StatusCode(404));
    let _ = request.respond(response);
}