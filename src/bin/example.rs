#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};
use uefi::Handle;
use uefi_exts::BootServicesExt as _;

#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).unwrap_success();

    let boot_services = system_table.boot_services();
    let gop = boot_services
        .find_handles::<GraphicsOutput>()
        .unwrap_success()
        .into_iter()
        .flat_map(|handle| {
            boot_services
                .handle_protocol::<GraphicsOutput>(handle)
                .warning_as_error()
                .ok()
        })
        .flat_map(|p| unsafe { p.get().as_mut() })
        .nth(0)
        .unwrap();

    let mode_info = gop.current_mode_info();
    gop.blt(BltOp::VideoFill {
        color: BltPixel::new(0, 0, 0),
        dest: (0, 0),
        dims: mode_info.resolution(),
    })
    .unwrap_success();

    uefi_unifont::print(gop, 0, 0, "Hello, world!");
    uefi_unifont::print(gop, 0, 32, "こんにちは世界！");
    uefi_unifont::print(gop, 0, 64, "色は匂へど　散りぬるを");
    uefi_unifont::print(gop, 0, 80, "我が世誰ぞ　常ならむ");
    uefi_unifont::print(gop, 0, 96, "有為の奥山　今日越えて");
    uefi_unifont::print(gop, 0, 112, "浅き夢見じ　酔ひもせず");

    Status::SUCCESS
}
