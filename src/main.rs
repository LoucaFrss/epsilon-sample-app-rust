#![no_std]
#![no_main]
#![feature(panic_info_message)]
pub mod eadk;
use eadk::prelude::*;
use eadk::{color::Color, random::random, Rect};
#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[no_mangle]
pub fn main() {
    for _ in 0..100 {
        let c: Color = random();
        let r: Rect = random();
        eadk::display::push_rect_uniform(r, c);
    }

    println!("Bonjour");
    loop {}
}
