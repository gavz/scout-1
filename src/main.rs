extern crate winapi;
extern crate winreg;
extern crate wmi;
extern crate proclist;

use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use wmi::{COMLibrary, Variant, WMIConnection};
use std::collections::HashMap;

mod processes;

fn main() {
    println!("\n\n================== PROCESSES ==================");

    let process_infos = proclist::iterate_processes_info().filter_map(|r| r.ok());
    for process_info in process_infos {
        match processes::lists::defensive_processes().iter().find(|&process| process.name.to_lowercase() == process_info.name.to_lowercase()) {
            Some(inner) => println!("[Defensive Process] ({}) {} - {}", process_info.pid, process_info.name, inner.description),
            None => (),
        }
        match processes::lists::interesting_processes().iter().find(|&process| process.name.to_lowercase() == process_info.name.to_lowercase()) {
            Some(inner) => println!("[Interesting Process] ({}) {} - {}", process_info.pid, process_info.name, inner.description),
            None => (),
        }
        match processes::lists::browser_processes().iter().find(|&process| process.name.to_lowercase() == process_info.name.to_lowercase()) {
            Some(inner) => println!("[Browser Process] ({}) {} - {}", process_info.pid, process_info.name, inner.description),
            None => (),
        }
    }

    println!("\n\n================== REGISTRY ==================");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion").unwrap();
    let pf: String = cur_ver.get_value("ProgramFilesDir").unwrap();
    let dp: String = cur_ver.get_value("DevicePath").unwrap();
    
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
    let info = cur_ver.query_info().unwrap();
    println!("info = {:?}", info);
    let mt = info.get_last_write_time_system();
    println!(
        "last_write_time as winapi::um::minwinbase::SYSTEMTIME = {}-{:02}-{:02} {:02}:{:02}:{:02}",
        mt.wYear, mt.wMonth, mt.wDay, mt.wHour, mt.wMinute, mt.wSecond
    );


    println!("\n\n===================== WMI ====================");
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::with_namespace_path("root\\SecurityCenter2", com_con.into()).unwrap();
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM AntiVirusProduct").unwrap();

    for av in results {
        println!("{:#?}", av);
    }
}
