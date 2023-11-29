use corrodedweb::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    let mut server = Server::new();
    server.set_document_root("./www/");
    server.use_index_of(true);
    server.set_logger("server.log");

    server.get("/parameter_demo/", |request, mut response| {
        let _ = response.set_status_code(200);
        let _ = response.write(
            "<html>
            Servus<br><br>
            QUERY Parameters
            <ul>",
        );
        for (k, v) in request.get_query_parameters().iter() {
            let _ = response.write(&format!("<li><b>{}</b> {}</li>", k, v));
        }
        let _ = response.write(
            "</ul><br>
            <form action='' method='POST'>
            First name: <input type='text' name='fname'><br><br>
            Last name: <input type='text' name='lname'><br><br>
            <input type='submit' value='Submit'></form>
            </html>",
        );
    });

    let counter = Arc::new(AtomicUsize::new(0));

    server.get("/counter/", move |_request, mut response| {
        let _ = response.set_status_code(200);
        let old = counter.load(Ordering::SeqCst);
        let _ = response.write(&format!("<h1>{}</h1>", old));
        counter.store(old + 1, Ordering::SeqCst);
    });

    server.post("/parameter_demo/", |request, mut response| {
        let _ = response.set_status_code(200);
        let _ = response.write(
            "<html>
            Hey why you POST me <br><br>
            POST Parameters
            <ul>",
        );
        for (k, v) in request.get_post_parameters().iter() {
            let _ = response.write(&format!("<li><b>{}</b> {}</li>", k, v));
        }
        let _ = response.write(
            "</ul><br>
            QUERY Parameters
            <ul>",
        );
        for (k, v) in request.get_query_parameters().iter() {
            let _ = response.write(&format!("<li><b>{}</b> {}</li>", k, v));
        }
        let _ = response.write(
            "<ul>
            </html>",
        );
    });

    server.get("/dead/", |_request, mut response| {
        let _ = response.set_status_code(200);
    });

    server.get("/idk/", |_, _| {
        // Do something without the user noticing
        println!("Hello");
    });

    server.start_server(7878);
}
