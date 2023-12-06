use crate::dsp_map::DspMap;
use std::io::{Read, Write};
use std::net::{TcpListener};
use httparse::Request;
use httparse;
use std::fs;

pub fn serve(map: DspMap) {
    let port = map.get("port").unwrap_or_else(|| panic!("Port not found on config.dsp"));
    let host = format!("127.0.0.1:{}", port);
    let server = TcpListener::bind(host.clone()).unwrap();
    println!("crème brûlée servie sur http://{host}");

    for stream in server.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 2048];

            if let Ok(_) = stream.read(&mut buffer) {
                if buffer.is_empty() {
                    continue
                }

                let mut headers = [httparse::EMPTY_HEADER; 16];
                let mut req = Request::new(&mut headers);

                if let Ok(request) = req.parse(&buffer) {
                    match request {
                        httparse::Status::Complete(_) => {
                            let path = req.path.unwrap_or("/");

                            let static_dir_render = map.get("static_dir_render")
                                .unwrap_or_else(|| panic!("Static dir render not found"));

                            let (resp_type, content) = if path.starts_with(static_dir_render) {
                                let static_dir = map.get("static_dir").unwrap_or_else(|| panic!("Static dir not found"));
                                let len = static_dir_render.len();
                                let mut file_path = static_dir.to_string();
                                file_path.insert_str(0, "./");
                                file_path.push('/');
                                file_path.push_str(&path[len..]);

                                match fs::read_to_string(&file_path) {
                                    Ok(content) => ("HTTP/1.1 200 OK\r\n", content),
                                    Err(_) => {
                                        let _app_html = map.get("_app_html").unwrap_or_else(|| panic!("_app_html not found"));

                                        match fs::read_to_string(format!("./{}", _app_html)) {
                                            Ok(content) => ("HTTP/1.1 200 OK\r\n", content),
                                            Err(_) => ("HTTP/1.1 404 Not Found\r\n", String::from("404 Page Not Found")),
                                        }
                                    },
                                }
                            } else {
                                let _app_html = map.get("_app_html").unwrap_or_else(|| panic!("_app_html not found"));

                                match fs::read_to_string(format!("./{}", _app_html)) {
                                    Ok(content) => ("HTTP/1.1 200 OK\r\n", content),
                                    Err(_) => ("HTTP/1.1 404 Not Found\r\n", String::from("404 Page Not Found")),
                                }
                            };

                            let response = format!("{}Content-Length: {}\r\n\r\n{}", resp_type, content.len(), content);
                            let _ = stream.write(response.as_bytes());
                        }
                        _ => {
                            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                            let _ = stream.write(response.as_bytes());
                        }
                    }
                } else {
                    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                    let _ = stream.write(response.as_bytes());
                }
            }
        }
    }
}
