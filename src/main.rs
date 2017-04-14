#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;

use hyper::Client;
use std::io::Read;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

#[derive(Serialize, Deserialize, Debug)]
struct Story {
    title: String,
    url: String,
    author: String,
    ups: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Child {
    data: Story,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    children: Vec<Child>,
    after: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    stories: Vec<Story>,
    after: String,
}

fn result_to_page(res: Result) -> Page {
    let stories: Vec<Story> = res.data.children.into_iter().map(|s| s.data).collect();
    return Page {
        stories: stories,
        after: res.data.after,
    };
}

trait RedditClient {
    fn get_top_reddit_page(&self, after: String) -> Page;
}

struct RedditHttpClient {
    client: Client,
}

impl RedditHttpClient {
    pub fn new() -> RedditHttpClient {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        RedditHttpClient {client: client}
    }
}

impl RedditClient for RedditHttpClient {
    fn get_top_reddit_page(&self, after: String) -> Page {
        let url = format!("https://www.reddit.com/r/all/top.json?after={}", after);
        let mut res = self.client.get(&url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let reddit_result: Result = serde_json::from_str(&body).unwrap();
        return result_to_page(reddit_result);
    }
}

fn main() {
    let reddit_client = RedditHttpClient::new();
    let page = reddit_client.get_top_reddit_page("foo".to_owned());
    println!("{:?}", page);
}
