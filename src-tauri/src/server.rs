use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, Responder};
use tauri::Emitter;

use crate::info::{Info, Test};
use crate::submit::*;
use crate::WINDOW;

pub struct WebState {
    pub info: Mutex<Option<Info>>,
    pub sol: Mutex<Option<Solution>>,
}

#[get("/getSubmit")]
pub async fn get_submit(data: web::Data<WebState>) -> impl Responder {
    let sol = data.sol.lock().unwrap().take();

    if sol.is_some() {
        let solution = sol.unwrap();

        #[cfg(debug_assertions)]
        println!("submitting solution");

        return HttpResponse::Ok().json(solution);
    }

    #[cfg(debug_assertions)]
    println!("no solution returning empty");

    HttpResponse::Ok().json(EmptySolution { empty: true })
}

#[post("/submit")]
pub async fn post_submit(sol: web::Json<Solution>, data: web::Data<WebState>) -> impl Responder {
    let _ = data.sol.lock().unwrap().insert(sol.0);

    #[cfg(debug_assertions)]
    println!("inserted solution into data");

    HttpResponse::Ok()
}

#[post("/")]
pub async fn post_info(req_body: web::Json<Info>, data: web::Data<WebState>) -> impl Responder {
    let _ = data.info.lock().unwrap().insert(req_body.0.clone());
    let window = WINDOW.get().expect("window-is-unavailable");
    window.emit("set-problem", req_body.get_problem()).unwrap();
    window
        .emit("set-verdicts", req_body.get_verdicts())
        .unwrap();
    HttpResponse::Ok()
}

#[get("/")]
pub async fn get_info(data: web::Data<WebState>) -> impl Responder {
    let info = data.info.lock().unwrap().clone().unwrap();
    HttpResponse::Ok().json(info)
}

#[post("/test_cases")]
pub async fn post_test_cases(cases: web::Json<Vec<Test>>) -> impl Responder {
    let cases = cases.0.clone();
    let verdicts = cases
        .into_iter()
        .map(|case| case.get_verdict())
        .collect::<Vec<_>>();
    let window = WINDOW.get().expect("window-is-unavailable");
    window
        .emit("add-verdicts", verdicts)
        .unwrap();
    HttpResponse::Ok()
}
