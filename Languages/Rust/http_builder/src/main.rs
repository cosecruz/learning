use std::slice::Windows;
use std::sync::WaitTimeoutResult;

enum Method {
    Get,
    Post,
}

struct Request {
    url: String,
    method: Method,
    // header: Vec<String>,
    body: Option<String>,
}

struct NoURL;
struct HasURL(String);
struct RequestBuilder<U> {
    url: U,
    method: Method,
    body: Option<String>,
}

impl RequestBuilder<NoURL> {
    fn new() -> Self {
        Self {
            url: NoURL,
            method: Method::Get,
            body: None,
        }
    }

    fn url(self, url: impl Into<String>) -> RequestBuilder<HasURL> {
        RequestBuilder {
            url: HasURL(url.into()),
            method: self.method,
            body: self.body,
        }
    }
}

//shared methods
impl<U> RequestBuilder<U> {
    fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

//builder only works on valid url
impl RequestBuilder<HasURL> {
    fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method,
            // header: ,
            body: self.body,
        }
    }
}

fn main() {
    let builder = RequestBuilder::new()
        .method(Method::Get)
        .url("https://api.example.com")
        .body("{}");

    let _request = builder.build();

    print_str("literal"); // &str
    print_str(String::from("owned")); // String
    print_str(&String::from("ref")); // &String
}

struct HugeData {
    data: [u8; 1024 * 1024],
}

trait Iterator {
    type Item<'a>
    where
        Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

fn handle_huge_data() {
    let hg = HugeData {
        data: [0; 1024 * 1024],
    };
}

struct WindowsMut<'data, T> {
    data: &'data mut [T],
    window_size: usize,
    pos: usize,
}

struct NotSend {
    x: i32,
    _marker: std::marker::PhantomData<*const ()>, // *const () is !Send
}

// Or explicitly:
//impl !Send for NotSend {} // Requires nightly

impl<'data, T> Iterator for WindowsMut<'data, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.pos + self.window_size > self.data.len() {
            return None;
        }

        let window = &mut self.data[self.pos..self.pos + self.window_size];
        self.pos += 1;
        Some(window)
    }
}

//blanket impl
//implement for all types that satisffy some bounds
fn print_str(s: impl AsRef<str>) {
    println!("{}", s.as_ref());
}

fn work() {}
