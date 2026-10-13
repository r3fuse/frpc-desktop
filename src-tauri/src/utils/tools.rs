use tauri::{AppHandle};
use tauri_plugin_shell::process::{ CommandChild};
use std::{ sync::Mutex};
use tauri::{Manager};
use crate::utils::proxies::{Config, Proxies};
use anyhow::{Result};

pub struct AppState {
    pub frpc_process: Mutex<Option<CommandChild>>,
    pub config: Mutex<crate::config_store::ConfigStore>
}

pub fn has_frpc_process(app_handle:&AppHandle) -> bool {
    let state = app_handle.state::<AppState>();
    let process_guard = state.frpc_process.lock().unwrap();
    println!("frpc process exists: {}", process_guard.is_some());  
    process_guard.is_some()
}

//删除配置
pub fn delete_proxy( app_handle: &AppHandle,name:String)->Result<(),String>{
   let state = app_handle.state::<AppState>();
   let mut data = state.config.lock().unwrap();
   data.delect_proxy(name);
   let _ = data.save();
   Ok(())
}

//修改配置
pub fn change_proxy( app_handle: &AppHandle,name:String,new_proxy:Proxies )->Result<(),String>{
    let state = app_handle.state::<AppState>();
    let mut data = state.config.lock().unwrap();
    data.change_proxy(name, new_proxy);
    let _ = data.save();
    Ok(())
}


pub fn check_name_is_exist(name:String,config:&Config)->bool{
    config.proxies.as_ref().is_some_and(|proxies| 
        {
            proxies.iter().any(|p| p.name ==name) 
        } 
    )
}
