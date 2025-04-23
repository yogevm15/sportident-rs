pub fn crc_slice(data: &[u8]) -> u16 {
    crc_iter(data.iter().copied())
}

pub fn crc_iter(mut bytes: impl Iterator<Item = u8>) -> u16 {
    const POLYNOM: u16 = 0x8005;

    // Read the first two bytes to initialize crc_val
    let Some(b0) = bytes.next() else { return 0 };
    let Some(b1) = bytes.next() else { return 0 };

    let mut crc_val = (u16::from(b0) << 8) | u16::from(b1);

    // Read the remaining bytes into a buffer of u16 values
    let iter = std::iter::from_fn(|| {
        let hi = bytes.next()?;
        let lo = bytes.next().unwrap_or(0); // pad with 0 if odd length
        Some((u16::from(hi) << 8) | u16::from(lo))
    });

    for mut ui_val in iter {
        for _ in 0..16 {
            if (crc_val & 0x8000) != 0 {
                crc_val = crc_val.wrapping_shl(1);
                if (ui_val & 0x8000) != 0 {
                    crc_val = crc_val.wrapping_add(1);
                }
                crc_val ^= POLYNOM;
            } else {
                crc_val = crc_val.wrapping_shl(1);
                if (ui_val & 0x8000) != 0 {
                    crc_val = crc_val.wrapping_add(1);
                }
            }
            ui_val = ui_val.wrapping_shl(1);
        }
    }

    crc_val
}
