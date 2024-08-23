use libatasmart::Disk;
use rocket::log::private::error;
use sysinfo::Disks;

pub fn smart_status_ok() -> bool {
    Disks::new_with_refreshed_list()
        .list()
        .iter()
        .filter_map(|disk| {
            Disk::new(disk.name().as_ref())
                .inspect_err(|error| error!("SMART: {error}"))
                .ok()
        })
        .all(|mut disk| disk.get_smart_status().unwrap_or(false))
}
