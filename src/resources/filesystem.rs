use alloc::sync;
use bevy::prelude::*;
use bevy_platform::sync::Arc;
use bevy_platform::sync::Mutex;
use esp_storage::FlashStorage;
use static_cell::StaticCell;
use littlefs2::fs::{
    Allocation,
    Filesystem
};
use crate::storage::LfsFlash;
use partitions::{
    PartitionType,
    DataPartitionSubType,
    PartitionEntry,
    FlashRegion
};
use esp_bootloader_esp_idf::partitions;
use esp_hal::peripherals::FLASH;

#[derive(Resource)]
pub struct FilesystemResource {
    // pub filesystem: Arc<Mutex<Filesystem<'static, LfsFlash<FlashRegion<'static, FlashStorage<'static>>>>>>
}

// static FLASH: StaticCell<FlashStorage> = StaticCell::new();
// static LFS_PT: StaticCell<PartitionEntry> = StaticCell::new();
// static LFS_STORAGE: StaticCell<LfsFlash<FlashRegion<FlashStorage>>> = StaticCell::new();
// static LFS_ALLOC: StaticCell<Allocation<LfsFlash<FlashRegion<FlashStorage>>>> = StaticCell::new();
// static mut PT_MEM: [u8; partitions::PARTITION_TABLE_MAX_LEN] = [0u8; partitions::PARTITION_TABLE_MAX_LEN];

// impl FilesystemResource {
//     pub fn new(flash_peripheral: FLASH<'static>) -> Self {
//         let mut flash = FLASH.init(FlashStorage::new(flash_peripheral));
    
//         let partition_table = unsafe {
//             partitions::read_partition_table(flash, &mut PT_MEM).unwrap()
//         };

//         let littlefs_storage = partition_table
//             .find_partition(PartitionType::Data(
//                 DataPartitionSubType::LittleFs,
//             ))
//             .unwrap()
//             .unwrap()
//             .as_embedded_storage(flash);

//         let filesystem_allocation = LFS_ALLOC.init(Filesystem::allocate());
//         let mut flash_storage = LFS_STORAGE.init(LfsFlash(littlefs_storage));
//         let filesystem = Filesystem::mount_or_else(filesystem_allocation, flash_storage, |_, s, _| {
//             Filesystem::format(s)
//         }).unwrap();

//         Self { filesystem: filesystem }
//     }
// }