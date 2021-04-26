use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    multipart::Form,
    redirect::Policy,
    Method, RequestBuilder, StatusCode, Url,
};
use serde::{de::DeserializeOwned, Serialize};

use std::{fmt, str::FromStr, time::Duration};

#[derive(Clone)]
struct Config {
    base_uri: Url,
    timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_uri: Url::from_str("https://api.chatwork.com/").unwrap(),
            timeout: Duration::from_secs(10),
        }
    }
}

pub(crate) struct PathAndQuery<'a, T> {
    pub path: &'a str,
    pub query: Option<&'a T>,
}

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    config: Config,
}

impl Client {
    #[cfg(feature = "default")]
    pub fn new(api_token: impl AsRef<str>) -> Self {
        let config = Config::default();

        let inner = reqwest::Client::builder()
            .user_agent(concat!(
                "github.com/mechiru/",
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("x-chatworktoken"),
                    HeaderValue::from_str(api_token.as_ref()).expect("invalid api token"),
                );
                headers
            })
            .redirect(Policy::none())
            .https_only(true)
            .timeout(config.timeout)
            .connect_timeout(config.timeout)
            .build()
            .unwrap();

        Self { inner, config }
    }

    #[cfg(not(feature = "default"))]
    pub fn new_with(client: reqwest::Client) -> Self {
        Self { inner: client, config }
    }
}

impl Client {
    fn url<Query>(&self, path_and_query: PathAndQuery<'_, Query>) -> crate::Result<Url>
    where
        Query: Serialize,
    {
        let mut url = self.config.base_uri.clone();
        url.set_path(path_and_query.path);
        if let Some(query) = path_and_query.query {
            let query = serde_urlencoded::to_string(query)?;
            url.set_query(Some(query.as_str()));
        }
        Ok(url)
    }

    async fn request<Query, Body, Model>(
        &self,
        method: Method,
        path_and_query: PathAndQuery<'_, Query>,
        body: Option<&Body>,
    ) -> crate::Result<Model>
    where
        Query: Serialize,
        Body: Serialize,
        Model: DeserializeOwned,
    {
        let mut req = self.inner.request(method, self.url(path_and_query)?);
        if let Some(body) = body {
            req = req.form(body);
        }
        self.handle(req).await
    }

    async fn handle<Model>(&self, req: RequestBuilder) -> crate::Result<Model>
    where
        Model: DeserializeOwned,
    {
        let resp = req.send().await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            code => Err(crate::Error::StatusCode(code)),
        }
    }

    pub(crate) async fn get<Query, Model>(
        &self,
        path_and_query: PathAndQuery<'_, Query>,
    ) -> crate::Result<Model>
    where
        Query: Serialize,
        Model: DeserializeOwned,
    {
        self.request(Method::GET, path_and_query, Option::<&()>::None).await
    }

    pub(crate) async fn post<Query, Body, Model>(
        &self,
        path_and_query: PathAndQuery<'_, Query>,
        body: &Body,
    ) -> crate::Result<Model>
    where
        Query: Serialize,
        Body: Serialize,
        Model: DeserializeOwned,
    {
        self.request(Method::POST, path_and_query, Some(body)).await
    }

    pub(crate) async fn put<Query, Body, Model>(
        &self,
        path_and_query: PathAndQuery<'_, Query>,
        body: Option<&Body>,
    ) -> crate::Result<Model>
    where
        Query: Serialize,
        Body: Serialize,
        Model: DeserializeOwned,
    {
        self.request(Method::PUT, path_and_query, body).await
    }

    pub(crate) async fn delete<Query, Body, Model>(
        &self,
        path_and_query: PathAndQuery<'_, Query>,
        body: Option<&Body>,
    ) -> crate::Result<Model>
    where
        Query: Serialize,
        Body: Serialize,
        Model: DeserializeOwned,
    {
        self.request(Method::DELETE, path_and_query, body).await
    }

    pub(crate) async fn multipart<Query, Model>(
        &self,
        path_and_query: PathAndQuery<'_, Query>,
        form: Form,
    ) -> crate::Result<Model>
    where
        Query: Serialize,
        Model: DeserializeOwned,
    {
        self.handle(self.inner.post(self.url(path_and_query)?).multipart(form)).await
    }
}

impl Client {
    pub fn v2(&self) -> crate::v2::Service {
        crate::v2::Service { inner: self.clone() }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client").finish()
    }
}
