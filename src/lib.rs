#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

#[macro_use]
extern crate log;

use alloc::sync::Arc;
use cv1811_sd::clk_en;
use devices::{
    device::{BlkDriver, DeviceType, Driver},
    driver_define,
    fdt::node::FdtNode,
};

pub struct CvSd;

unsafe impl Sync for CvSd {}
unsafe impl Send for CvSd {}

impl Driver for CvSd {
    fn get_id(&self) -> &str {
        "cvitek,sd"
    }

    fn get_device_wrapper(self: Arc<Self>) -> DeviceType {
        DeviceType::BLOCK(self.clone())
    }
}

impl BlkDriver for CvSd {
    fn read_blocks(&self, block_id: usize, buf: &mut [u8]) {
        assert!(
            buf.len() % 0x200 == 0,
            "can't read data notaligned 0x200 in the kcvitek-sd"
        );
        clk_en(true);
        for i in 0..buf.len() / 0x200 {
            let start = i * 0x200;
            cv1811_sd::read_block((block_id + i) as _, &mut buf[start..start + 0x200])
                .expect("can't read block by using CvSd");
        }
        clk_en(false);
    }

    fn write_blocks(&self, block_id: usize, buf: &[u8]) {
        assert!(
            buf.len() % 0x200 == 0,
            "can't write data notaligned 0x200 in the kcvitek-sd"
        );
        clk_en(true);
        for i in 0..buf.len() / 0x200 {
            let start = i * 0x200;
            cv1811_sd::write_block((block_id + i) as _, &buf[start..start + 0x200])
                .expect("can't write block by using CvSd");
        }
        clk_en(false);
    }
}

pub fn init_driver(_node: &FdtNode) -> Arc<dyn Driver> {
    let blk = CvSd;
    cv1811_sd::init().expect("init with err");
    info!("Initailize virtio-block device");
    Arc::new(blk)
}

driver_define!("cvitek,mars-sd", init_driver);
