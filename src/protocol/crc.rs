pub fn crc(data: &[u8]) -> u16 {
    const POLYNOM: u16 = 0x8005;

    let n = data.len();
    if n < 2 {
        return 0;
    }

    // Start by reading the first two bytes as the initial CRC value.
    let mut index = 0;
    let mut crc_val = (u16::from(data[index]) << 8) | u16::from(data[index + 1]);
    index += 2;

    // For exactly two bytes, return immediately.
    if n == 2 {
        return crc_val;
    }

    // Calculate the number of iterations as the number of 16-bit groups.
    // Note: The loop counter is computed as n >> 1 (i.e. floor(n/2))
    let num_iters = n >> 1;

    // Iterate in reverse order so that the last iteration (i == 1) can handle the odd byte case.
    for i in (1..=num_iters).rev() {
        // In all iterations except the last, read two bytes.
        // In the last iteration, if there's an odd number of bytes, pad with a zero.
        let mut ui_val = if i > 1 {
            let value = (u16::from(data[index]) << 8) | u16::from(data[index + 1]);
            index += 2;
            value
        } else if n & 1 == 1 {
            let value = u16::from(data[index]) << 8;
            index += 1;
            value
        } else {
            0
        };

        // Process 16 shifts per 16-bit group.
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
