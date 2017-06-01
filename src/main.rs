#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate rocket;
extern crate rocket_contrib;
extern crate livesplit_core;
extern crate parking_lot;

mod component;
mod config;

use std::io::BufReader;
use std::fs::File;
use rocket_contrib::JSON;
use rocket::State;
use rocket::config::{Config, Environment};
use livesplit_core::{Timer, SharedTimer, HotkeySystem};
use livesplit_core::parser::composite::parse;
use parking_lot::RwLock;
use component::{Component, ComponentState};

struct ServerState {
    timer: SharedTimer,
    components: Vec<Component>,
}

#[get("/")]
fn index(state: State<RwLock<ServerState>>) -> JSON<Vec<ComponentState>> {
    let mut state = state.write();
    let state: &mut ServerState = &mut state;
    let timer = state.timer.read();
    let components = &mut state.components;

    JSON(components.iter_mut().map(|c| c.state(&timer)).collect())
}

#[get("/split")]
fn split(state: State<RwLock<ServerState>>) -> JSON<Vec<ComponentState>> {
    {
        let state = state.write();
        state.timer.write().split();
    }

    index(state)
}

fn main() {
    let config = config::load();

    let file = BufReader::new(File::open(&config.splits).unwrap());
    let run = parse(file, Some(config.splits), true).unwrap();

    let timer = Timer::new(run).into_shared();

    let _hotkey_system = if config.hotkeys {
        HotkeySystem::new(timer.clone()).ok()
    } else {
        None
    };

    let server_state = ServerState {
        timer: timer,
        components: config
            .layout
            .0
            .into_iter()
            .map(|c| c.into())
            .collect(),
    };

    let rocket_config = Config::build(Environment::Production)
        .address(config.address)
        .port(config.port)
        .unwrap();

    rocket::custom(rocket_config, true)
        .manage(RwLock::new(server_state))
        .mount("/", routes![index, split])
        .launch();
}
