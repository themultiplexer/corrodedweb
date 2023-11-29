//! A web-framework-crate written in Rust
//!
//! This crate is a fast and versatile "web-server" similar to the package [Express](https://expressjs.com/de/)
//! for NodeJS.
//!
//! ### Static Files
//! The main feature of a webserver is the serving of files from the filesystem.
//! You can tell corrodedweb to serve all files in a specific directory online.
//!
//! More precisly you can set which files are beeing served when only a directory
//! is specified e.g. `moodle.htwg-konstanz.de/moodle/`. These will be index files
//! like index.html or index.txt.
//!
//! ### User Defined Routing
//! Users can specify callbacks which will be called when the application receives
//! a request to the specified route (endpoint) and HTTP method. With `post(...)`
//! and `get(...)` you can handle POST and GET request to any route you like.
//! ```ignore
//! corroded.get("/home/", |request, response| {
//!   response.send("Homepage")
//! })
//! ```
//!
//! ### Multithreading
//! For seamless usage of functionality multithreading is indispensable.
//! Corrodedweb itself is multithreaded.

/// Logs everything
mod logger;
/// The main module
mod server;
/// Manages workers of the webserver
mod threadpool;

pub use logger::Logger;
pub use server::Server;
