// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_shell::ShellExt;
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::process::{ Command, CommandChild};
use tauri::{path::BaseDirectory,Manager};
use std::io::Write;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;
mod utils;
use utils::tools::{has_frpc_process,mget_resource_path,generate_config,change_proxy,delete_proxy,write_proxy,parsing_config,AppState};
use utils::proxies::{Config,Proxies,MsgType,ProxyType};
use memory_stats::memory_stats;
use sysinfo::{
    Components, Disks, Networks, Pid
};

use crate::utils::proxies::MessageType::{self, Error, Success};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app|{
            app.manage(AppState { frpc_process: Mutex::new(None) });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_frp,stop_frp,open_config_window,add_proxy,delete_config,change_config_by_name,get_config,create_window,get_momery_usage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


//向前端页面发送通知
#[tauri::command]
fn send_notification(app_handle: &AppHandle,msg:MsgType) {
    app_handle
        .emit("notification",msg)
        .unwrap();
}


//启动frpc进程
#[tauri::command]
fn start_frp(app_handle: AppHandle) {
    if has_frpc_process(&app_handle){
        let title = "FRP is already running".to_string();
        let message = "Please stop the existing FRP process before starting a new one.".to_string();
        let msg = MsgType{msg_type:MessageType::Error,title,message};
        send_notification(&app_handle, msg);
        return;
    }
    // let resource_path = app_handle.path().resolve("resources/frpc.toml", BaseDirectory::Resource).expect("Failed to resolve resource path");
    // let resource_path = get_resource_path(&app_handle);
    // println!("Resolved resource path: {:?}", resource_path);

    let path = match mget_resource_path(&app_handle){
        Ok(path)=>path,
        Err(e)=>{
            eprintln!("{:?}",e);
            return;
        }
    };

    let sider_command = app_handle.shell().sidecar("frpc").expect("REASON").args(&["-c",&path]);
    let (mut rx, child) = sider_command.spawn().expect("Failed to spawn frpc process");
    
    let state = app_handle.state::<AppState>();
    let mut frpc_process_lock = state.frpc_process.lock().unwrap();
    *frpc_process_lock = Some(child);
    drop(frpc_process_lock);

    tauri::async_runtime::spawn(async move{
        while let Some(event) = rx.recv().await{
            if let CommandEvent::Stdout(line_bytes) = event{
                let line = String::from_utf8_lossy(&line_bytes);
                println!("frpc output: {}", &line);
                app_handle.emit("frp-log", line.to_string()).unwrap();
            }
        }
    });
    
}



//终止frpc进程
#[tauri::command]
fn stop_frp(app_handle: tauri::AppHandle) {
    let state = app_handle.state::<AppState>();
    let mut process_guard = state.frpc_process.lock().unwrap();
    
    if let Some(child) = process_guard.take() {
        match child.kill() {
            Ok(_) => {
                println!("frpc process stopped successfully");
                let _ = app_handle.emit("frp-log", "frpc stopped".to_string());
            }
            Err(e) => {
                eprintln!("Failed to kill frpc process: {}", e);
                let _ = app_handle.emit("frp-log", format!("Failed to stop frpc: {}", e));
            }
        }
    } else {
        println!("No frpc process running");
        let _ = app_handle.emit("frp-log", "No frpc process is running".to_string());
    }
}


#[tauri::command]
async fn open_config_window(app_handle: AppHandle){
    let config_window = tauri::WebviewWindowBuilder::new(&app_handle, "local", tauri::WebviewUrl::App("index.html".into())).build().unwrap();
}


#[tauri::command]
fn add_proxy(app_handle: AppHandle,config:Proxies){
    let mut f = fs::File::options().append(true).open("F:\\rwx\\test.toml").expect("读取错误");

    let cfg: Result<String, String> = generate_config(config);
    
    let cfg = match cfg{
        Ok(v)=>v,
        Err(e)=>{
            println!("配置生成异常，{}",e);
            panic!("配置生成异常")
        },
    };

    match f.write_all(&cfg.as_bytes()){
        Ok(s)=>{
            println!("file write successfully!");
            let title = String::from("配置发生变动");
            let msg = String::from("新增成功");
            let m = MsgType{msg_type:MessageType::Success,title:title,message:msg};
            send_notification(&app_handle, m);
            // println!("{:?}", app_handle.webview_windows());
            app_handle.emit("config_updated",{}).unwrap();
            if let Some(win) = app_handle.get_webview_window("config"){
                let _ = win.close();
            };
        },
        Err(e)=>println!("error:{}",e)
    };
    // parsing_config();
}


//获取解析配置文件
#[tauri::command]
async fn get_config(app_handle: AppHandle,name:String){
    let config = parsing_config().await;
    match config {
        Ok(c)=>{
            app_handle.emit("get_config", &c).unwrap();
            println!("get name parameters:{}",name);
        },
        Err(e)=>{
            println!("{}",e);
        }
    };
    
//     let change_result =  change_config(name,&mut config);
//    let result = match change_result {
//        Ok(_)=>{send_notification(&app_handle, "success".to_string(), "修改通知".to_string(), "配置修改成功".to_string())},
//        Err(_)=>{send_notification(&app_handle, "success".to_string(), "修改通知".to_string(), "配置修改成功".to_string())},
//    };

}

// 修改配置
#[tauri::command]
async fn change_config_by_name(app_handle: AppHandle, name:String,proxy:Proxies){
    let mut config = parsing_config().await.unwrap();
    match change_proxy(name, &mut config, proxy){
        Ok(())=>{
            let win = app_handle.get_webview_window("config").unwrap();
            match write_proxy(&config, "F:\\rwx\\test.toml".to_string()).await{
                Ok(())=>{
                    println!("写入成功");
                    app_handle.emit("config_updated", {});
                    let msg = MsgType{msg_type:MessageType::Success,title:"通知".to_string(),message:"修改成功".to_string()};
                    let _ = win.close();
                    send_notification(&app_handle, msg);
                },
                Err(e)=>{
                    println!("写入失败:{}",e.to_string());
                    let msg = MsgType{msg_type:MessageType::Error,title:"通知".to_string(),message:"修改失败".to_string()};
                    let _ = win.close();
                    send_notification(&app_handle, msg);
                }
            }         
        },
        Err(e)=>{
            println!("修改失败:{}",e);
        }
    }
}

#[tauri::command]
async fn delete_config(app_handle:AppHandle,proxy:Proxies){
    let mut file_cfg = parsing_config().await.unwrap();
    let result = delete_proxy(proxy.name, &mut file_cfg);
    println!("exec delect function after:{:?}",&file_cfg);
    let win = app_handle.get_webview_window("config").unwrap();
    match result {
        Ok(s)=>{
            match write_proxy(&file_cfg,"F:\\rwx\\test.toml".to_string()).await{
                Ok(())=>{
                    let msg = MsgType{msg_type:MessageType::Success,title:"通知".to_string(),message:"删除成功".to_string()};
                    let _ =win.close();
                    send_notification(&app_handle, msg);
                    app_handle.emit("config_updated", {}).unwrap();
                },
                Err(e)=>{
                    let msg = MsgType{msg_type:MessageType::Error,title:"通知".to_string(),message:e.to_string()};
                    let _ = win.close();
                    send_notification(&app_handle, msg);
                    return ;
                }
            } 
            
        },
        Err(e)=>{
            let msg = MsgType{msg_type:MessageType::Error,title:"通知".to_string(),message:e};
            let _ = win.close();
            send_notification(&app_handle, msg);
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
enum FRPConfigOperation{
    Create,Edit,Delete
}

#[tauri::command]
async fn create_window(app: tauri::AppHandle,operation:FRPConfigOperation,proxy:Proxies) {
    let url = format!("/#/config/{:?}",operation);
    let webview_window = tauri::WebviewWindowBuilder::new(&app, "config", tauri::WebviewUrl::App(url.into()))
        .title("配置管理")
        .resizable(false)
        .inner_size(360.0, 250.0)
        .center()
        .build()
        .unwrap();
    app.emit("proxy", proxy);
}

#[tauri::command]
fn get_momery_usage(){
        // if let Some(usage) = memory_stats(){
        //     println!("physical memory:{}",usage.virtual_mem);
        //     (usage.virtual_mem,usage.physical_mem)
        // }else {
        //     println!("Can't get memory usage");
        //     (0,0)
        // }
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
       match sysinfo::get_current_pid() {
            Ok(pid) => {
                println!("current pid: {}", pid);
            }
            Err(e) => {
                println!("failed to get current pid: {}", e);
            }
        }
}
