mod plan_generator;
use std::{thread, sync::mpsc,};
use actix_web::{post, web::{Json, self}, App, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
struct ResponseJSON {
    message : String,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum RequestJSON {
    leds_on(Vec<usize>),
    leds_off(Vec<usize>),
    all_leds_on(bool),
    all_leds_off(bool),
    leds_blink(Vec<usize>),
}

#[post("/api/led_control")]
async fn led_control(body: Json<RequestJSON>, send_channel: web::Data<mpsc::Sender<plan_generator::Message>>) -> Result<impl Responder> {
    let body = body.into_inner();
    let try_handle_request = match body {
        RequestJSON::leds_on(leds) => {
            send_channel.send(plan_generator::Message::LedsOn(leds)).err()
        }
        RequestJSON::leds_off(leds) => {
            send_channel.send(plan_generator::Message::LedsOff(leds)).err()
        }
        RequestJSON::all_leds_on(state) => {
            if state { send_channel.send(plan_generator::Message::AllLedsOn).err() } else { None }
        }
        RequestJSON::all_leds_off(state) => {
            if state { send_channel.send(plan_generator::Message::AllLedsOff).err() } else { None }
        }
        RequestJSON::leds_blink(leds) => {
            send_channel.send(plan_generator::Message::LedsBlink(leds)).err()
        }
    };
    match try_handle_request {
        Some(_e) => panic!("[Error] Could not send message to Plan Generator."),
        _ => {}
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
