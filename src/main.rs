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
    ups : i64,
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
    after : String,
}

fn result_to_page(res:Result) -> Page {
    let stories :Vec<Story> = res.data.children.into_iter().map(|s|s.data).collect();
    return Page {stories : stories, after : res.data.after}
}


fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut res = client.get("https://www.reddit.com/r/all/top.json").send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let reddit_result: Result = serde_json::from_str(&body).unwrap();
    let page = result_to_page(reddit_result);
    println!("{:?}", page);
}
