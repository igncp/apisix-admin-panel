use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    get,
    http::StatusCode,
    post, web, App, HttpResponse, HttpServer, Responder,
};
use admin_api_handler::AdminApiHandler;
use admin_standalone_handler::AdminStandaloneHandler;
use apisix_admin_panel_core::proxy::{ProxyFetchMethod, ProxyFetchOpts};
use config::ServerConfig;
use serde::{Deserialize, Serialize};
use std::{io::Read, sync::Arc};
use ts_rs::TS;

mod admin_api_handler;
mod admin_standalone_handler;
mod config;

type HandlerConfig = web::Data<Arc<ServerConfig>>;

#[post("/api/apisix-admin")]
async fn post_proxy_apisix_admin(
    config: HandlerConfig,
    body: web::Json<ProxyFetchOpts>,
) -> impl Responder {
    let text = if let Some(standalone_config_path) = config.standalone_config_path.clone() {
        let handler = AdminStandaloneHandler {
            config_path: standalone_config_path,
        };
        handler.handle(body.into_inner()).await
    } else {
        let handler = AdminApiHandler {
            api_key: config.api_key.clone(),
            url: config.admin_url.clone(),
        };
        handler.handle(body.into_inner()).await
    };

    if let Ok(text) = text {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(text);
    }

    println!("Error: {:?}", text);

    HttpResponse::InternalServerError().body("Error fetching data from APISIX admin API")
}

#[post("/api/apisix-control")]
async fn post_proxy_apisix_control(
    config: HandlerConfig,
    body: web::Json<ProxyFetchOpts>,
) -> impl Responder {
    let client = reqwest::Client::new();

    let url = format!("{}{}", config.control_url.clone(), body.uri);

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

#[get("/api/apisix-config")]
async fn get_apisix_config(config: HandlerConfig) -> impl Responder {
    let apisix_key = config.api_key.clone();

    let file = NamedFile::open(config.config_file_path.clone());

    if file.is_err() {
        return HttpResponse::NotFound().body("APISIX config file not found");
    }

    let mut file = file.unwrap();

    let mut content = String::new();
    let read_result = file.read_to_string(&mut content);

    if read_result.is_err() {
        return HttpResponse::InternalServerError().body("Failed to read APISIX config file");
    }

    read_result.unwrap();

    let sanitized_content = content.replace(&apisix_key, "REDACTED");
    let parsed_content: Result<serde_yaml::Value, _> = serde_yaml::from_str(&sanitized_content);

    if parsed_content.is_err() {
        return HttpResponse::InternalServerError().body("Failed to parse APISIX config file");
    }

    let parsed_content = parsed_content.unwrap();
    let json_content = serde_json::to_string(&parsed_content);

    if json_content.is_err() {
        return HttpResponse::InternalServerError().body("Failed to convert APISIX config to JSON");
    }

    let json_content = json_content.unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/yaml"))
        .body(json_content)
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
struct ServerInfo {
    is_standalone: bool,
}

#[get("/api/info")]
async fn get_info(config: HandlerConfig) -> impl Responder {
    let server_info = ServerInfo {
        is_standalone: config.standalone_config_path.is_some(),
    };
    let json_str = serde_json::to_string(&server_info).unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/yaml"))
        .body(json_str)
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

        let config = web::Data::new(Arc::new(ServerConfig::new()));

        App::new()
            .app_data(config)
            .service(get_apisix_config)
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
