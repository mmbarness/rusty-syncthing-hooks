#![feature(provide_any)]
#![feature(error_generic_member_access)]
#![feature(trait_alias)]
#![feature(async_closure)]
use std::{thread::JoinHandle, process::Child};

use syncthing::{configs, logger::{Logger, ErrorLogging}};
use tokio::{ time };
use log::{error, info};

use crate::syncthing::spawn_script::{Spawn, Scripts, ScriptsError};
mod syncthing;

#[tokio::main]
async fn main() {
    match tokio::task::spawn(async {
        poll().await;
    }).await {
        Ok(_) => (),
        Err(_) => {
            Logger::log_error_string(&"error executing poll()".to_string());
        }
    }
}

async fn poll() {
    match tokio::spawn(async move {
        Logger::on_load();
        let api_configs = match configs::Configs::load() {
            Ok(c) => c,
            Err(e) => {
                error!("error loading configs: {}", e.to_string());
                panic!()
            }
        };
        let scripts_spawn_record = match Scripts::ingest_configs() {
            Ok(script_records) => script_records,
            Err(e) => {
                error!("error loading configs: {}", e.to_string());
                panic!()
            }
        };

        let mut interval = time::interval(time::Duration::from_secs(api_configs.request_interval.clone()));
        let mut syncthing_api = syncthing::api::SyncthingApi::new(api_configs);
        info!("beginning to poll...");
        loop {
            let (updated_api, events) = match syncthing_api.update().await {
                Ok((updated_api, events)) => (updated_api.clone(), events),
                Err(e) => {
                    error!("{}", e.to_string());
                    return;
                }
            };

            match (&events.len() > &0) {
                true => {
                    let spawn_errors:Vec<Result<Option<Vec<JoinHandle<Result<Child, ScriptsError>>>>, ScriptsError>> = events.into_iter().map(|e| {
                        let event_type = e.r#type.clone();
                        info!("running event of type: {}", event_type);
                        Scripts::run_event(&scripts_spawn_record, event_type)
                    }).collect();
                },
                false => {
                    info!("no events...");
                }
            };


            syncthing_api = updated_api;
            
            interval.tick().await;
        }
    }).await {
        Ok(_) => (),
        Err(_) => {
            error!("error spawning loop")
        }
    }
}