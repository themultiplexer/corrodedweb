# Corrodedweb
A web-framework-crate written in Rust

## Introduction
This crate provides a fast and versatile "web-server" similar to the package [Express](https://expressjs.com/de/)
for NodeJS. The main features are Static Files, User Defined Routing and Multithreading.

## Functionality
- [X] Logging (logging files, logging statistics)

- [x] Static Files

- [x] User Defined Routing

- [x] Multithreading inside of Corrodedweb

### Static Files
The main feature of a webserver is the serving of files from the filesystem.
You can tell *corrodedweb* to serve all files in a specific directory and provide default index files.

### User Defined Routing
Users can specify callbacks which will be called when the application receives
a request to the specified route (endpoint) and HTTP method. With `post(...)`
and `get(...)` you can handle POST and GET request to any route you like.

```rust
corroded.get('/home/', |request, response| {
  response.send('Homepage')
})
```

### Logging
To enhance the usage experience logging is necessary. The logging should be
complete but concise. Personal logging paths are possible. Logging statistics
provide a fast overview about what happened in recent history.

## Dependencies
- humantime = "1.2.0"
- reqwest = "0.9.18"
