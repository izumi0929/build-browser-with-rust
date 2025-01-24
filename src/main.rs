#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use noli::*;
use saba_core::browser::Browser;
use saba_core::http::HttpResponse;

static TEST_HTTP_RESPONSE: &str = r#"HTTP/1.1 200 OK
Data: Wed, 21 Oct 2015 07:28:00 GMT

<html>
<head></head>
<body>
    <h1 id="title">Hello, World!</h1>
    <h2 class="class">H2 title</h2>
    <p>Paragraph</p>
    <p>
        <a href="https://www.google.com">Google</a>
        <a href="https://www.bing.com">Bing</a>
    </p>
</body>
</html>
"#;


fn main() -> u64 {
    let browser = Browser::new();

    let response = HttpResponse::new(TEST_HTTP_RESPONSE.to_string()).expect("failed to parse http response");
    let page = browser.borrow().current_age();
    let dom_string = page.borrow_mut().receive_response(response);

    for log in dom_string {
        println!("{}", log);
    }

    0
}

entry_point!(main);