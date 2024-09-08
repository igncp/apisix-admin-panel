use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    get,
    http::StatusCode,
    post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use admin_api_handler::AdminApiHandler;
use admin_standalone_handler::AdminStandaloneHandler;
use apisix_admin_panel_core::proxy::{ProxyFetchMethod, ProxyFetchOpts};
use auth::{get_auth_scope, verify_auth};
use config::{HandlerConfig, ServerConfig};
use serde::{Deserialize, Serialize};
use server_error::{CommonResponse, RespError};
use std::{io::Read, sync::Arc};
use ts_rs::TS;

mod admin_api_handler;
mod admin_standalone_handler;
mod auth;
mod config;
mod server_error;

#[post("/api/apisix-admin")]
async fn post_proxy_apisix_admin(
    req: HttpRequest,
    config: HandlerConfig,
    body: web::Json<ProxyFetchOpts>,
) -> CommonResponse {
    verify_auth(&req, &config)?;

    let text = (if let Some(standalone_config_path) = config.standalone_config_path.clone() {
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
    })
    .map_err(|_| RespError::Custom("Error fetching data from APISIX admin API".to_string()))?;

    return Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(text));
}

#[post("/api/apisix-control")]
async fn post_proxy_apisix_control(
    req: HttpRequest,
    config: HandlerConfig,
    body: web::Json<ProxyFetchOpts>,
) -> CommonResponse {
    verify_auth(&req, &config)?;

    let client = reqwest::Client::new();

    let url = format!("{}{}", config.control_url.clone(), body.uri);

    match body.method {
        ProxyFetchMethod::GET => {
            let res = client.get(&url).send().await.unwrap();

            return Ok(HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(res.text().await.unwrap()));
        }
        ProxyFetchMethod::PUT => {
            let mut client = client.put(&url);

            if let Some(data) = &body.data {
                client = client.body(data.clone());
            }

            let res = client.send().await.unwrap();

            Ok(HttpResponse::Ok().json(res.text().await.unwrap()))
        }
        _ => Ok(HttpResponse::BadRequest().body("Method not allowed")),
    }
}

#[get("/api/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/api/apisix-config")]
async fn get_apisix_config(req: HttpRequest, config: HandlerConfig) -> CommonResponse {
    verify_auth(&req, &config)?;

    let apisix_key = config.api_key.clone();

    let mut file = NamedFile::open(config.config_file_path.clone())
        .map_err(|_| RespError::Custom("APISIX config file not found".to_string()))?;

    let mut content = String::new();

    file.read_to_string(&mut content)
        .map_err(|_| RespError::Custom("Failed to read APISIX config file".to_string()))?;

    let sanitized_content = content.replace(&apisix_key, "REDACTED");
    let parsed_content: serde_yaml::Value = serde_yaml::from_str(&sanitized_content)
        .map_err(|_| RespError::Custom("Failed to parse APISIX config file".to_string()))?;
    let json_content = serde_json::to_string(&parsed_content)
        .map_err(|_| RespError::Custom("Failed to convert APISIX config to JSON".to_string()))?;

    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/yaml"))
        .body(json_content))
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
struct ServerInfo {
    apisix_url: String,
    is_standalone: bool,
    has_auth: bool,
}

#[get("/api/info")]
async fn get_info(req: HttpRequest, config: HandlerConfig) -> CommonResponse {
    verify_auth(&req, &config)?;

    let server_info = ServerInfo {
        apisix_url: config.apisix_url.clone(),
        has_auth: !config.jwt_secret.is_empty(),
        is_standalone: config.standalone_config_path.is_some(),
    };
    let json_str = serde_json::to_string(&server_info).unwrap();

    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/yaml"))
        .body(json_str))
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
        let auth_scope = get_auth_scope();

        App::new()
            .app_data(config)
            .service(auth_scope)
            .service(get_apisix_config)
            .service(get_health)
            .service(get_info)
            .service(post_proxy_apisix_admin)
            .service(post_proxy_apisix_control)
            .service(static_files)
            .wrap(cors)
    })
    .bind((address, port))?
    .run()
    .await
}
