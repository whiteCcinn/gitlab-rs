use reqwest::Client;

pub trait HttpClient {
    fn get(&self, url: String);
    fn post(&self, url: String);
    fn put(&self, url: String);
    fn delete(&self, url: String);
}

impl HttpClient for Client {
    fn get(&self, url: String) {
        println!("get {}", url)
    }

    fn post(&self, url: String) {
        println!("post {}", url)
    }

    fn put(&self, url: String) {
        println!("put {}", url)
    }

    fn delete(&self, url: String) {
        println!("delete {}", url)
    }
}