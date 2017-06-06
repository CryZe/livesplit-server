#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate rocket;
extern crate rocket_contrib;
extern crate livesplit_core;
extern crate parking_lot;

mod layout;
mod config;

use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use rocket_contrib::JSON;
use rocket::State;
use rocket::config::{Config, Environment};
use livesplit_core::{Timer, Run, Segment, SharedTimer, HotkeySystem};
use livesplit_core::parser::composite::parse;
use livesplit_core::saver::livesplit;
use parking_lot::RwLock;
use layout::{Component, ComponentState};

struct ServerState {
    timer: SharedTimer,
    current_splits: String,
    config: config::Config,
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

#[get("/reset")]
fn reset(state: State<RwLock<ServerState>>) -> JSON<Vec<ComponentState>> {
    {
        let state = state.write();
        state.timer.write().reset(true);
    }

    index(state)
}

#[get("/undo-split")]
fn undo_split(state: State<RwLock<ServerState>>) -> JSON<Vec<ComponentState>> {
    {
        let state = state.write();
        state.timer.write().undo_split();
    }

    index(state)
}

#[get("/skip-split")]
fn skip_split(state: State<RwLock<ServerState>>) -> JSON<Vec<ComponentState>> {
    {
        let state = state.write();
        state.timer.write().skip_split();
    }

    index(state)
}

fn load_splits_from_name(config: &config::Config, name: &str) -> Result<Run, &'static str> {
    let path = config
        .splits
        .get(name)
        .cloned()
        .ok_or("Splits Name doesn't match any splits in the list")?;

    let file = File::open(&path)
        .map_err(|_| "Couldn't find splits file")?;

    parse(BufReader::new(file), Some(path), true).map_err(|_| "Couldn't parse splits file")
}

#[get("/splits/load/<name>")]
fn load_splits(state: State<RwLock<ServerState>>,
               name: String)
               -> Result<JSON<Vec<ComponentState>>, &'static str> {
    {
        let mut state = state.write();
        let run = load_splits_from_name(&state.config, &name)?;
        *state.timer.write() = Timer::new(run);
        state.current_splits = name;
    }

    Ok(index(state))
}

#[get("/splits/save")]
fn save_splits(state: State<RwLock<ServerState>>)
               -> Result<JSON<Vec<ComponentState>>, &'static str> {
    {
        let state = state.read();
        let path = state
            .config
            .splits
            .get(&state.current_splits)
            .cloned()
            .unwrap_or_else(|| PathBuf::from("splits.lss"));

        let file = File::create(path)
            .map_err(|_| "Splits file couldn't be created")?;

        let timer = state.timer.read();
        livesplit::save(timer.run(), BufWriter::new(file))
            .map_err(|_| "Couldn't save splits file")?;
    }

    Ok(index(state))
}

fn main() {
    let mut config = config::load();

    let run = match load_splits_from_name(&config, &config.default_splits) {
        Ok(run) => run,
        Err(e) => {
            println!("Warning while loading default splits:");
            println!("{}", e);
            let mut run = Run::new();
            run.set_game_name("Sample Game");
            run.set_category_name("Sample Category");
            run.push_segment(Segment::new("Sample Segment"));
            run
        }
    };

    let timer = Timer::new(run).into_shared();

    let _hotkey_system = if config.hotkeys {
        HotkeySystem::new(timer.clone()).ok()
    } else {
        None
    };

    let components = config.layout.0.drain(..).map(|c| c.into()).collect();

    let rocket_config = Config::build(Environment::Production)
        .address(config.address.clone())
        .port(config.port)
        .unwrap();

    let current_splits = config.default_splits.clone();

    let server_state = ServerState {
        timer,
        config,
        components,
        current_splits,
    };

    rocket::custom(rocket_config, true)
        .manage(RwLock::new(server_state))
        .mount("/",
               routes![index,
                       split,
                       undo_split,
                       skip_split,
                       reset,
                       load_splits,
                       save_splits])
        .launch();
}
