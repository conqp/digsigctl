use std::collections::HashMap;

use rocket::serde::json::serde_json;
use serde::Serialize;
use sysinfo::Disks;

use application::Metadata;
use cmdline::cmdline;
use cpuinfo::CpuInfo;
use df::Entry;
use efi::Efi;
use meminfo::meminfo;
use mount::root_mounted_ro;
use sensors::sensors;
use uptime::Uptime;

use crate::sysinfo::smart::smart_status_ok;

mod application;
mod cmdline;
mod cpuinfo;
mod df;
mod efi;
mod meminfo;
mod mount;
mod sensors;
mod smart;
mod uptime;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum Os {
    Unix,
    Windows,
}

#[derive(Debug, Serialize)]
pub struct SystemInformation {
    os: Os,
    application: Metadata,
    baytrail: Option<bool>,
    efi: Efi,
    #[serde(rename = "cmdline")]
    cmd_line: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "cpuinfo")]
    cpu_info: Option<CpuInfo>,
    df: Vec<Entry>,
    #[serde(rename = "meminfo")]
    mem_info: Option<HashMap<String, usize>>,
    root_ro: Option<bool>,
    sensors: Option<serde_json::Value>,
    uptime: Uptime,
    smart_ok: bool,
}

impl Default for SystemInformation {
    fn default() -> Self {
        Self {
            #[cfg(target_family = "unix")]
            os: Os::Unix,
            #[cfg(target_family = "windows")]
            os: Os::Windows,
            application: application::status(),
            baytrail: CpuInfo::read().map(|cpu_info| cpu_info.is_bay_trail()).ok(),
            efi: Efi::default(),
            cmd_line: cmdline().ok(),
            cpu_info: CpuInfo::read().ok(),
            df: Disks::new_with_refreshed_list()
                .list()
                .iter()
                .map(Entry::from)
                .collect(),
            mem_info: meminfo().ok(),
            root_ro: root_mounted_ro().ok(),
            sensors: sensors().ok(),
            uptime: Uptime::default(),
            smart_ok: smart_status_ok(),
        }
    }
}
