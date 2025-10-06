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
    println!("javaslatok: {}", fs::read_to_string("javaslatok.txt").unwrap());
    let port = 3030;
    println!("running on port {}", port);
    let help = warp::path!("help").map(|| "help here");
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

    let javaslat = warp::post()
        .and(warp::path("javaslat"))
        .and(warp::body::form())
        .map(|jav: HashMap<String, String>|{
            println!("{}", &jav["javaslat"]);
            newquote("javaslatok", &jav["javaslat"]);
            warp::reply::html("elmentve")
        });
    let javaslatok = warp::path("javaslatok")
        .map(||{
            fs::read_to_string("javaslatok.txt").unwrap()
        });

    let zenek = warp::path("zenek")
        .map(||{
            warp::reply::html(fs::read_to_string("zenek.html").unwrap())
        });

    let read_uj = warp::path("quotesof_uj")
        .and(warp::path::param())
        .map(|param: String| {
            //fs::read_to_string(param+".txt").unwrap()
            warp::reply::html(fs::read_to_string("quotesof.html").unwrap().replace(":?", &fs::read_to_string(param+".txt").unwrap()))
        });

    let routes = help.or(home).or(idezet).or(read).or(javaslat).or(javaslatok).or(zenek).or(read_uj);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

fn newquote(kie: &str, vicces: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(kie.to_owned()+".txt").unwrap();
    file.write_all(format!("\n{}", vicces).as_bytes()).unwrap();
}
fn prepare(quotes: &str) {
    let mut sliced: Vec<&str>= quotes.split("\n").collect::<Vec<&str>>().iter().map(|quote|{
        *quote
    }).collect();
}
