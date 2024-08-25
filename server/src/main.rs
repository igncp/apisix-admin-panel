use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    get,
    http::StatusCode,
    post, web, App, HttpResponse, HttpServer, Responder,
};
use apisix_admin_panel_core::proxy::{ProxyFetchMethod, ProxyFetchOpts};

#[post("/api/apisix-admin")]
async fn post_proxy_apisix_admin(body: web::Json<ProxyFetchOpts>) -> impl Responder {
    let client = reqwest::Client::new();

    let url =
        std::env::var("APISIX_ADMIN_URL").unwrap_or_else(|_| "http://localhost:9180".to_string());
    let api_key = std::env::var("APISIX_API_KEY")
        .unwrap_or_else(|_| "edd1c9f034335f136f87ad84b625c8f1".to_string());

    let url = format!("{}/apisix/admin{}", url, body.uri);

    match body.method {
        ProxyFetchMethod::GET => {
            let res = client
                .get(&url)
                .header("X-API-KEY", api_key)
                .send()
                .await
                .unwrap();

            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(res.text().await.unwrap());
        }
        ProxyFetchMethod::POST => {
            let res = client
                .post(&url)
                .header("X-API-KEY", api_key)
                .body(body.data.clone().unwrap())
                .send()
                .await
                .unwrap();

            HttpResponse::Ok().json(res.text().await.unwrap())
        }
        ProxyFetchMethod::PUT => {
            let res = client
                .put(&url)
                .header("X-API-KEY", api_key)
                .body(body.data.clone().unwrap())
                .send()
                .await
                .unwrap();

            HttpResponse::Ok().json(res.text().await.unwrap())
        }
        ProxyFetchMethod::DELETE => {
            let res = client
                .delete(&url)
                .header("X-API-KEY", api_key)
                .send()
                .await
                .unwrap();

            HttpResponse::Ok().json(res.text().await.unwrap())
        }
    }
}

#[post("/api/apisix-control")]
async fn post_proxy_apisix_control(body: web::Json<ProxyFetchOpts>) -> impl Responder {
    let client = reqwest::Client::new();

    let url =
        std::env::var("APISIX_CONTROL_URL").unwrap_or_else(|_| "http://localhost:9090".to_string());

    let url = format!("{}{}", url, body.uri);

    match body.method {
        ProxyFetchMethod::GET => {
            let res = client.get(&url).send().await.unwrap();

            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(res.text().await.unwrap());
        }
        ProxyFetchMethod::PUT => {
            let mut client = client.put(&url);

            if let Some(data) = &body.data {
                client = client.body(data.clone());
            }

            let res = client.send().await.unwrap();

            HttpResponse::Ok().json(res.text().await.unwrap())
        }
        _ => HttpResponse::BadRequest().body("Method not allowed"),
    }
}

#[get("/api/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let default_port = 9000;
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| default_port.to_string())
        .parse()
        .unwrap_or(9000);
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Starting the APISIX HTTP server on http://{address}:{port}");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let static_files = Files::new("/", "./static")
            .index_file("index.html")
            .redirect_to_slash_directory()
            .default_handler(fn_service(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();
                let file = NamedFile::open_async("./static/404/index.html").await?;

                let res = file
                    .customize()
                    .with_status(StatusCode::NOT_FOUND)
                    .respond_to(&req)
                    .map_into_boxed_body();

                Ok(ServiceResponse::new(req, res))
            }));

        App::new()
            .service(get_health)
            .service(post_proxy_apisix_admin)
            .service(post_proxy_apisix_control)
            .service(static_files)
            .wrap(cors)
    })
    .bind((address, port))?
    .run()
    .await
}
