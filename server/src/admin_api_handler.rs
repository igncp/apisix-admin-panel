use apisix_admin_panel_core::proxy::{ProxyFetchMethod, ProxyFetchOpts};

pub struct AdminApiHandler {
    pub api_key: String,
    pub url: String,
}

impl AdminApiHandler {
    pub async fn handle(&self, opts: ProxyFetchOpts) -> Result<String, &'static str> {
        let url = format!("{}/apisix/admin{}", self.url.clone(), opts.uri);
        let client = reqwest::Client::new();
        let common_error = "Error fetching data from APISIX admin API";

        let res = match opts.method {
            ProxyFetchMethod::GET => client
                .get(&url)
                .header("X-API-KEY", self.api_key.clone())
                .send()
                .await
                .map_err(|_| common_error)?,
            ProxyFetchMethod::POST => client
                .post(&url)
                .header("X-API-KEY", self.api_key.clone())
                .body(opts.data.clone().unwrap())
                .send()
                .await
                .map_err(|_| common_error)?,
            ProxyFetchMethod::PATCH => client
                .patch(&url)
                .header("X-API-KEY", self.api_key.clone())
                .body(opts.data.clone().unwrap())
                .send()
                .await
                .map_err(|_| common_error)?,
            ProxyFetchMethod::PUT => client
                .put(&url)
                .header("X-API-KEY", self.api_key.clone())
                .body(opts.data.clone().unwrap())
                .send()
                .await
                .map_err(|_| common_error)?,
            ProxyFetchMethod::DELETE => client
                .delete(&url)
                .header("X-API-KEY", self.api_key.clone())
                .send()
                .await
                .map_err(|_| common_error)?,
        };

        res.text().await.map_err(|_| common_error)
    }
}
