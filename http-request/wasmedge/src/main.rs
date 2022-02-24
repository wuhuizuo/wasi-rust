use wasmedge_http_req::request;

// can only request with plain http to ip address.

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("http://14.215.177.38", &mut writer).unwrap();

    println!("GET");
    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    println!("{}", String::from_utf8_lossy(&writer));
}
