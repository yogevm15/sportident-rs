#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
use std::ffi::{c_int, c_short, c_uchar, c_uint, c_ushort};

pub fn crc(buf: &[u8]) -> u16 {
    unsafe { generated_crc(buf.len() as c_uint, buf.as_ptr() as *const c_uchar) as u16 }
}

unsafe extern "C" fn generated_crc(ui_count: c_uint, puc_dat: *const c_uchar) -> c_uint {
    let mut i_tmp: c_short;
    let mut ui_tmp: c_ushort;
    let mut ui_tmp1: c_ushort;
    let mut ui_val: c_ushort;
    let mut puc_tmp_dat: *const c_uchar;
    if ui_count < 2 as c_int as c_uint {
        return 0 as c_int as c_uint;
    }
    puc_tmp_dat = puc_dat;
    let fresh0 = puc_tmp_dat;
    puc_tmp_dat = puc_tmp_dat.offset(1);
    ui_tmp1 = *fresh0 as c_ushort;
    let fresh1 = puc_tmp_dat;
    puc_tmp_dat = puc_tmp_dat.offset(1);
    ui_tmp1 = (((ui_tmp1 as c_int) << 8 as c_int) + *fresh1 as c_int) as c_ushort;
    if ui_count == 2 as c_int as c_uint {
        return ui_tmp1 as c_uint;
    }
    i_tmp = (ui_count >> 1 as c_int) as c_int as c_short;
    while i_tmp as c_int > 0 as c_int {
        if i_tmp as c_int > 1 as c_int {
            let fresh2 = puc_tmp_dat;
            puc_tmp_dat = puc_tmp_dat.offset(1);
            ui_val = *fresh2 as c_ushort;
            let fresh3 = puc_tmp_dat;
            puc_tmp_dat = puc_tmp_dat.offset(1);
            ui_val = (((ui_val as c_int) << 8 as c_int) + *fresh3 as c_int) as c_ushort;
        } else if ui_count & 1 as c_int as c_uint != 0 {
            ui_val = *puc_tmp_dat as c_ushort;
            ui_val = ((ui_val as c_int) << 8 as c_int) as c_ushort;
        } else {
            ui_val = 0 as c_int as c_ushort;
        }
        ui_tmp = 0 as c_int as c_ushort;
        while (ui_tmp as c_int) < 16 as c_int {
            if ui_tmp1 as c_int & 0x8000 as c_int != 0 {
                ui_tmp1 = ((ui_tmp1 as c_int) << 1 as c_int) as c_ushort;
                if ui_val as c_int & 0x8000 as c_int != 0 {
                    ui_tmp1 = ui_tmp1.wrapping_add(1);
                }
                ui_tmp1 = (ui_tmp1 as c_int ^ 0x8005 as c_int) as c_ushort;
            } else {
                ui_tmp1 = ((ui_tmp1 as c_int) << 1 as c_int) as c_ushort;
                if ui_val as c_int & 0x8000 as c_int != 0 {
                    ui_tmp1 = ui_tmp1.wrapping_add(1);
                }
            }
            ui_val = ((ui_val as c_int) << 1 as c_int) as c_ushort;
            ui_tmp = ui_tmp.wrapping_add(1);
        }
        i_tmp -= 1;
    }
    ui_tmp1 as c_uint
}
