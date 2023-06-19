use std::{thread, sync::{mpsc, Mutex},};
use actix_web::{post, web::{Json, self}, App, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
mod plan_generator;

#[derive(Serialize)]
struct ResponseJSON {
    message : String,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum RequestJSON {
    leds_on(Vec<u8>),
    leds_off(Vec<u8>),
    all_leds_on(bool),
    all_leds_off(bool),
    leds_blink(Vec<u8>),
}

#[post("/api/led_control")]
async fn led_control(body: Json<RequestJSON>, send_channel: web::Data<mpsc::Sender<plan_generator::Message>>) -> Result<impl Responder> {
    let body = body.into_inner();
    match body {
        RequestJSON::leds_on(leds) => {
            println!("leds_on {:?}", leds);
        }
        RequestJSON::leds_off(leds) => {
            println!("leds_off {:?}", leds);
        }
        RequestJSON::all_leds_on(state) => {
            println!("all_leds_on {state}");
        }
        RequestJSON::all_leds_off(state) => {
            println!("all_leds_off {state}");
        }
        RequestJSON::leds_blink(leds) => {
            println!("leds_blink {:?}", leds);
        }
    }
    Ok(Json(ResponseJSON { message : "Success!".to_string() }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(set_up_plan_generator()))
            .service(led_control)
    })
    .bind(("localhost", 5000))?
    .run()
    .await
}    


fn set_up_plan_generator() -> mpsc::Sender<plan_generator::Message> {
    let (send_end, receive_end) = mpsc::channel();
    thread::spawn(move || {
        plan_generator::start(receive_end);
    });
    send_end
}
