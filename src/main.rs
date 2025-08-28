use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use warp::{filters::any, reply::reply, Filter};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
enum kie{
    vili,
    patrik,
    imre,
    pisti
}

#[derive(Deserialize, Debug)]
struct Vicces {
    kie: kie,
    vicces: String,
}

#[tokio::main]
async fn main() {
    let port = 3030;
    println!("running on port {}", port);
    let hello = warp::path!("hello" / String).map(|name| format!("hello {}", name));
    let help = warp::path!("help").map(|| "help here");
    let asdf = warp::path!("asdf").map(|| "asdf here");
    let home =
        warp::path::end().map(|| warp::reply::html(fs::read_to_string("index.html").unwrap()));
    let idezet = warp::post()
        .and(warp::path("idezet"))
        .and(warp::body::form())
        //.and(warp::body::content_length_limit(1024*120))
        .map(|body: HashMap<String, String>|{ // aranykopes: Vicces
            //println!("{}", aranykopes.vicces);
            newquote(&body["kie"], &body["vicces"]);
            println!("{}",body["kie"]);
            println!("{}",body["vicces"]);
            warp::reply::html("elmentve")
            //warp::reply::reply()
        });

    let read = warp::path("quotesof")
        .and(warp::path::param())
        .map(|param: String| {
            fs::read_to_string(param+".txt").unwrap()
        });

    let routes = help.or(asdf).or(hello).or(home).or(idezet).or(read);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn newquote(kie: &str, vicces: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(kie.to_owned()+".txt").unwrap();
    file.write_all(format!("\n{}", vicces).as_bytes()).unwrap();
}
