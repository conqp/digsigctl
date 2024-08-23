use libatasmart::Disk;
use sysinfo::Disks;

pub fn smart_status_ok() -> bool {
    Disks::new_with_refreshed_list().list().iter().all(|disk| {
        Disk::new(disk.name().as_ref())
            .expect("failed to create disk")
            .get_smart_status()
            .unwrap_or(false)
    })
}
