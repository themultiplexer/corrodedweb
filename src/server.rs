use crate::logger::Logger;
use crate::threadpool::ThreadPool;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::str;
use std::sync::{Arc, Mutex};

/// Represents the data which was sent by the caller
pub struct Request {
    post_parameters: HashMap<String, String>,
    query_parameters: HashMap<String, String>,
}

impl Request {
    fn new() -> Self {
        Request {
            post_parameters: HashMap::new(),
            query_parameters: HashMap::new(),
        }
    }
    /// Returns POST parameters of this request
    pub fn get_post_parameters(&self) -> HashMap<String, String> {
        self.post_parameters.clone()
    }
    /// Returns GET (query) parameters of this request
    ///
    /// http://localhost:7878/`?test=123&hallo=3` will return a HashMap:
    /// ```ignore
    /// {
    ///  "test" : "123",
    ///  "hallo" : "3"
    /// }
    /// ```
    pub fn get_query_parameters(&self) -> HashMap<String, String> {
        self.query_parameters.clone()
    }
}

/// Allows you to send data back to the client
pub struct Response {
    stream: TcpStream,
}

impl Response {
    fn new(stream: TcpStream) -> Self {
        Response { stream }
    }
    /// Write data into the response. Will be flushed no later than on drop.
    pub fn write(&mut self, data: &str) -> std::io::Result<()> {
        self.stream.write_all(data.as_bytes())
    }
    /// Set the status code of the response
    pub fn set_status_code(&mut self, code: u32) -> std::io::Result<()> {
        let response = format!("HTTP/1.1 {} OK\r\n\r\n", code);
        self.stream.write_all(response.as_bytes())
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        let _ = self.stream.flush();
    }
}

type Callback = Box<dyn Fn(Request, Response) + Send + Sync>;

/// Represents the web-framemorks server. The most important struct.
pub struct Server {
    document_root: Option<PathBuf>,
    logger: Option<Logger>,
    index_of: bool,
    registered_endpoints: Arc<Mutex<HashMap<(String, String), Callback>>>,
}

impl Server {
    /// Returns a Server instance
    ///
    /// # Example
    ///
    /// ```
    /// use corrodedweb::Server;
    /// let c = Server::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the document root after it is tested by test_document_root()
    /// function and returns true if successfull
    ///
    /// # Arguments
    ///
    /// * `document_root` - A string slice that holds the absolute or relative
    /// path to the document root
    ///
    /// # Example
    ///
    /// ```
    /// use corrodedweb::Server;
    /// let mut s = Server::new();
    /// s.set_document_root("/path/to/document/root");
    /// s.set_document_root("../path/to/document/root");
    /// ```
    pub fn set_document_root(&mut self, document_root: &str) -> bool {
        match self.test_document_root(document_root) {
            Some(root) => {
                if let Some(path) = root.to_str() {
                    Logger::info(
                        &self.logger,
                        &format!("New document_root was set to {}", path),
                    );
                } else {
                    Logger::warning(
                        &self.logger,
                        "New document_root is not UTF-8 valid",
                    );
                }
                self.document_root = Some(root);
                true
            }
            None => false,
        }
    }

    /// Returns the Option containing a valid path as PathBuf or None if invalid
    ///
    /// # Example
    ///
    /// ```
    /// use corrodedweb::Server;
    /// let mut s = Server::new();
    /// match s.get_document_root() {
    ///     Some(path) => println!("Path is set to {:?}", path),
    ///     None => println!("No path is set!"),
    /// };
    /// ```
    pub fn get_document_root(&self) -> Option<PathBuf> {
        self.document_root.clone()
    }

    /// Sets whether to show a list of files, when navigating to a folder
    pub fn use_index_of(&mut self, index_of: bool) {
        self.index_of = index_of;
    }

    /// Tests whether document root is valid an return an Option
    fn test_document_root(&mut self, document_root: &str) -> Option<PathBuf> {
        let mut path_to_root = PathBuf::new();
        path_to_root.push(document_root);
        if path_to_root.exists() {
            Some(path_to_root)
        } else {
            Logger::warning(
                &self.logger,
                &format!("document_root {} is not valid", document_root),
            );
            None
        }
    }

    /// Sets a logger for the server instance with specific log path
    ///
    /// # Arguments
    ///
    /// * `log_path` - Path to log file
    ///
    /// # Example
    ///
    /// ```
    /// let mut s = corrodedweb::Server::new();
    /// s.set_logger("./file.log")
    /// ```
    pub fn set_logger(&mut self, log_path: &str) {
        self.logger = Some(Logger::new(log_path));
    }

    /// Registers for a GET-request
    ///
    ///
    /// # Arguments
    ///
    /// * `route` - The endpoint you will register to.
    /// * `f` - The callback closure which will be executed on request.
    ///
    /// # Example
    ///
    /// ```
    /// use corrodedweb::Server;
    /// let mut s = Server::new();
    /// s.get("/", |request, mut response| {
    ///
    /// });
    /// ```
    pub fn get<F: Send + Sync + 'static>(&mut self, route: &str, f: F)
    where
        F: Fn(Request, Response),
    {
        self.registered_endpoints
            .lock()
            .unwrap()
            .insert((String::from(route), String::from("GET")), Box::new(f));
        Logger::info(
            &self.logger,
            &format!("Registered route: {}, method: {}", route, "GET"),
        );
    }

    /// Registers for a POST-request
    ///
    ///
    /// # Arguments
    ///
    /// * `route` - The endpoint you will register to.
    /// * `f` - The callback closure which will be executed on request.
    ///
    /// # Example
    ///
    /// ```
    /// use corrodedweb::Server;
    /// let mut s = Server::new();
    /// s.post("/", |request, mut response| {
    ///
    /// });
    /// ```
    pub fn post<F: Send + Sync + 'static>(&mut self, route: &str, f: F)
    where
        F: Fn(Request, Response),
    {
        self.registered_endpoints
            .lock()
            .unwrap()
            .insert((String::from(route), String::from("POST")), Box::new(f));
        Logger::info(
            &self.logger,
            &format!("Registered route: {}, method: {}", route, "POST"),
        );
    }

    /// Starts serving your files or listening for your registered enpoints.
    ///
    /// # Arguments
    ///
    /// * `port` - The port the server will listen on
    ///
    /// ```
    /// use corrodedweb::Server;
    /// let mut s = Server::new();
    /// // Blocks, so it is commented out
    /// // s.start_server(7878);
    /// ```
    pub fn start_server(&self, port: u32) {
        if let Ok(listener) = TcpListener::bind(&format!("127.0.0.1:{}", port)) {
            Logger::info(
                &self.logger,
                &format!("Open TCP Port {} for incomming connections", port),
            );

            let threadpool = ThreadPool::new(8);

            for stream in listener.incoming() {
                let s = self.clone();
                if let Ok(stream) = stream {
                    threadpool.execute(move || {
                        s.handle_connection(stream);
                    });
                }
            }
        }
    }

    fn parse_parameters(parameter_string: Option<&&str>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let parameters: Vec<&str> = if let Some(string) = parameter_string {
            if string.is_empty() {
                Vec::new()
            } else {
                string.split('&').collect()
            }
        } else {
            Vec::new()
        };
        for param in parameters {
            let kv_pair: Vec<&str> = param.split('=').collect();
            map.insert(
                kv_pair.get(0).unwrap_or(&"").to_string(),
                kv_pair.get(1).unwrap_or(&"").to_string(),
            );
        }
        map
    }

    /// Handles a connection and writes to a TcpStream
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            Logger::warning(&self.logger, format!("Error: {}", e).as_str())
        }

        if let Ok(s) = str::from_utf8(&buffer) {
            let si = s.replace("\u{0}", "");
            let header_lines: Vec<&str> = si.split("\r\n").collect();
            let header: Vec<&str> = header_lines[0].split(' ').collect();

            if header.len() > 1 {
                let url_with_params: Vec<&str> = header[1].split('?').collect();
                let request = String::from(url_with_params[0]);

                Logger::debug(
                    &self.logger,
                    &format!("header: {}, request: {}", header[0], request),
                );

                if let Some(callback) = self
                    .registered_endpoints
                    .lock()
                    .unwrap()
                    .get(&(request, header[0].to_string()))
                {
                    // User registered for this route, call their callback
                    Logger::info(&self.logger, "Users custom route hit");

                    let response = Response::new(stream);
                    let mut request = Request::new();
                    request.post_parameters = Server::parse_parameters(header_lines.last());
                    request.query_parameters = Server::parse_parameters(url_with_params.get(1));

                    callback.deref()(request, response);
                } else if let Some(path) = &self.document_root {
                    self.serve_static_files(&mut stream, path, header[1]);
                }
            }
        }
    }

    /// Serves static files
    fn serve_static_files(&self, stream: &mut TcpStream, path: &PathBuf, virtual_path: &str) {
        let v_path = virtual_path.trim_start_matches('/');

        let mut write_to_stream = |bytes| {
            if let Err(e) = stream.write_all(bytes) {
                Logger::warning(&self.logger, format!("Error: {}", e).as_str());
            }
            if let Err(e) = stream.flush() {
                Logger::warning(&self.logger, format!("Error: {}", e).as_str());
            }
        };

        let requested_path = format!(
            "{}{}",
            path.clone().into_os_string().into_string().unwrap(),
            v_path,
        );

        if Path::new(&requested_path).exists() {
            if Path::new(&requested_path).is_file() {
                Logger::info(
                    &self.logger,
                    &format!("Requested file {} exists", requested_path),
                );
                let mut buf = Vec::new();
                match File::open(&requested_path) {
                    Ok(mut content) => {
                        match content.read_to_end(&mut buf) {
                            Ok(bytes_read) => {
                                Logger::info(
                                    &self.logger,
                                    format!("\t{} bytes were read", bytes_read).as_str(),
                                );
                            }
                            Err(e) => {
                                Logger::warning(&self.logger, format!("Error: {}", e).as_str());
                            }
                        };
                    }
                    Err(e) => {
                        Logger::warning(&self.logger, format!("Error: {}", e).as_str());
                    }
                };
                let ok = String::from("HTTP/1.1 200 OK\r\n\r\n");
                let response = ok.as_bytes();
                write_to_stream(&[response, buf.as_slice()].concat());
            } else if Path::new(&requested_path).is_dir() && self.index_of {
                Logger::info(
                    &self.logger,
                    &format!("Requested path {} is directory", requested_path),
                );
                let index_of = Server::generate_index_of(&requested_path, v_path);
                write_to_stream(format!("HTTP/1.1 200 OK\r\n\r\n{}", index_of).as_bytes());
            }
        } else {
            Logger::info(&self.logger, "Status 404: Not found");
            write_to_stream(
                format!(
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n{}",
                    "<html><h1>404 not found</h1><hr> powered by corrodedweb</html>"
                )
                .as_bytes(),
            );
        }
    }

    fn generate_index_of(path: &str, virtual_path: &str) -> String {
        let paths = fs::read_dir(path).unwrap();
        let mut index_of = String::new();
        index_of.push_str(&format!(
            "<html>Index of <b>/{}</b><br><br><ul>",
            virtual_path
        ));
        index_of.push_str("<li><a href='..'>..</li>");
        for path in paths {
            let path = path.unwrap().path();
            let file_name = &path.file_name().unwrap().to_string_lossy();
            index_of.push_str(&format!(
                "<li><a href='{}/{}'>{}</li>",
                virtual_path, file_name, file_name
            ));
        }
        index_of.push_str("</ul></html>");
        index_of
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            document_root: None,
            logger: None,
            index_of: false,
            registered_endpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            document_root: self.document_root.clone(),
            logger: self.logger.clone(),
            index_of: self.index_of,
            registered_endpoints: self.registered_endpoints.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_get() {
        let mut server = Server::new();
        server.get("/", |request, mut response| {
            let parameters = request.get_query_parameters();
            assert_eq!(parameters.get("param1"), Some(&String::from("hello")));
            assert_eq!(parameters.get("param2"), Some(&String::from("1234")));
            let _ = response.set_status_code(200);
            let _ = response.write("123456789");
        });

        thread::spawn(move || {
            server.start_server(7878);
        });

        loop {
            if let Ok(mut resp) = reqwest::get("http://localhost:7878/?param1=hello&param2=1234") {
                assert_eq!(resp.status().is_success(), true);
                assert_eq!(resp.text().unwrap(), String::from("123456789"));
                break;
            }
        }
    }

    #[test]
    fn test_post() {
        let mut server = Server::new();
        server.post("/post/", |_request, mut response| {
            let _ = response.set_status_code(200);
            let _ = response.write("123456789");
        });

        thread::spawn(move || {
            server.start_server(7879);
        });

        loop {
            let client = reqwest::Client::new();
            if let Ok(mut resp) = client.post("http://localhost:7879/post/").send() {
                assert_eq!(resp.status().is_success(), true);
                assert_eq!(resp.text().unwrap(), String::from("123456789"));
                break;
            }
        }
    }
}
