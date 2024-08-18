#[cfg(test)]
mod test {
    use std::f32::consts::E;

    #[derive(Debug)]
    struct Task {
        pub title: String,
        pub done: bool,
        pub description: Option<String>,
    }

    impl Task {
        pub fn new(title: impl Into<String>, description: impl Into<String>) -> Task {
            Task {
                title: title.into(),
                done: false,
                description: Some(description.into()),
            }
        }
    }

    impl Default for Task {
        fn default() -> Self {
            Task {
                title: "Untitled".to_string(),
                done: false,
                description: None,
            }
        }
    }

    #[test]
    fn basic_create() {
        let task = Task {
            title: "Learn Rust".to_string(),
            done: false,
            description: Some("Learn Rust from the practice".to_string()),
        };
        println!("{:?}", task);
    }

    #[test]
    fn basic_new() {
        let task = Task::new("Learn Rust", "Learn Rust from the practice");
        println!("{:?}", task);
    }

    #[test]
    fn default_task() {
        let task = Task::default();
        println!("{:?}", task);
    }

    #[test]
    fn another_default_task() {
        let task = Task {
            description: "Learn Rust from the practice".to_string().into(),
            ..Default::default()
        };
        println!("{:?}", task);
    }

    #[test]
    fn unwrap_or_default() {
        let task: Option<Task> = None;
        let task = task.unwrap_or_default();
        println!("{:?}", task);
    }

    #[derive(Debug)]
    pub struct Request {
        pub url: String,
        pub method: String,
        pub headers: Vec<(String, String)>,
        pub body: Option<String>,
    }

    #[derive(Default, Clone)]
    pub struct RequestBuilder {
        url: Option<String>,
        method: Option<String>,
        headers: Vec<(String, String)>,
        body: Option<String>,
    }

    impl RequestBuilder {
        pub fn new() -> Self {
            RequestBuilder::default()
        }

        pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
            self.url = Some(url.into());
            self
        }

        pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
            self.method = Some(method.into());
            self
        }

        pub fn header(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
            self.headers.push((key.into(), value.into()));
            self
        }

        pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
            self.body = Some(body.into());
            self
        }

        pub fn build(&self) -> Result<Request, &str> {
            let Some(url) = self.url.as_ref() else {
                return Err("URL is required");
            };

            let method = self
                .method
                .as_ref()
                .cloned()
                .unwrap_or_else(|| "GET".to_string());

            Ok(
                Request {
                    url: url.clone(),
                    method,
                    headers: self.headers.clone(),
                    body: self.body.clone(),
                }
            )
        }

        pub fn build_once(&mut self) -> Result<Request, &str> {
            let Some(url) = self.url.take() else {
                return Err("URL is required");
            };

            let method = self
                .method
                .take()
                .unwrap_or_else(|| "GET".to_string());

            Ok(
                Request {
                    url,
                    method,
                    headers: self.headers.clone(),
                    body: self.body.take(),
                }
            )
        }

        pub fn url_consuming(mut self, url: impl Into<String>) -> Self {
            self.url = Some(url.into());
            // self.url.insert(url.into());
            self
        }

        pub fn method_consuming(mut self, method: impl Into<String>) -> Self {
            self.method = Some(method.into());
            self
        }

        pub fn header_consuming(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.headers.push((key.into(), value.into()));
            self
        }

        pub fn body_consuming(mut self, body: impl Into<String>) -> Self {
            self.body = Some(body.into());
            self
        }

        pub fn build_consuming(self) -> Result<Request, &'static str> {
            let Some(url) = self.url else {
                return Err("URL is required");
            };

            let method = self
                .method
                .unwrap_or_else(|| "GET".to_string());

            Ok(
                Request {
                    url,
                    method,
                    headers: self.headers,
                    body: self.body,
                }
            )
        }
    }

    #[test]
    fn basic_request_builder() {
        let request = RequestBuilder::new()
            .url("https://www.rust-lang.org")
            .method("GET")
            .header("User-Agent", "Rust")
            .header("Content-Type", "text/html")
            .body("Hello, Rust!")
            .build()
            .unwrap();
        println!("{:?}", request);

        let mut request_builder = RequestBuilder::new();
        request_builder.url("https://www.rust-lang.org");
        request_builder.body("Hello, Rust!");
        let request = request_builder.build().unwrap();
        println!("{:?}", request);
        request_builder.header("User-Agent", "Rust");
        let request = request_builder.build().unwrap();
        println!("{:?}", request);
    }

    #[test]
    fn consuming_request_builder() {
        let request_builder = RequestBuilder::new()
            .url_consuming("https://www.rust-lang.org")
            .method_consuming("GET");

        let request_builder = request_builder
            .header_consuming("User-Agent", "Rust")
            .header_consuming("Content-Type", "text/html");
        
        let request = request_builder
            .body_consuming("Hello, Rust!")
            .build_consuming()
            .unwrap();

        println!("{:?}", request);
    }

    #[test]
    fn consuming_request_builder_clone() {
        let request_builder = RequestBuilder::new()
            .url_consuming("https://www.rust-lang.org")
            .method_consuming("GET");

        let request_builder = request_builder
            .header_consuming("User-Agent", "Rust")
            .header_consuming("Content-Type", "text/html");
        
        let request = request_builder
            .clone()
            .body_consuming("Hello, Rust!")
            .build_consuming()
            .unwrap();
        println!("{:?}", request);

        let request = request_builder
            .body_consuming("Hello, Ruby!")
            .build_consuming()
            .unwrap();
        println!("{:?}", request);
    }
}
