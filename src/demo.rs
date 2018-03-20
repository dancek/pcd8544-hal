use Pcd8544;
use core::num::Wrapping;

type Buffer = [u8; 504];

static RUST_LOGO: &'static Buffer = include_bytes!("logo.bin");

fn empty_buffer() -> Buffer {
    [0u8; 504]
}

pub fn demo(pcd8544: &mut Pcd8544) {
    loop {
        run_shader(pcd8544, 0..135, deform_1_z);

        for _ in 0..20 { pcd8544.draw_buffer(RUST_LOGO); }

        run_optimized_mandelbrot(pcd8544);

        for _ in 0..20 { pcd8544.draw_buffer(RUST_LOGO); }
    }
}

pub fn apply_shader<F: Fn(i32, i32, i32) -> bool>(buffer: &mut Buffer, t: i32, f: F) {
    for col in 0..84 {
        for row in 0..6 {
            let x = col as i32 - 42;
            let mut byte = 0x00;
            for bit in 0..8 {
                let y = 8 * (row as i32 - 3) + bit;
                byte += (f(x, y, t) as u8) << bit;
            }
            buffer[6 * col + row] = byte;
        }
    }
}

pub fn run_shader<F: Fn(i32, i32, i32) -> bool>(
    pcd8544: &mut Pcd8544,
    t_range: ::core::ops::Range<i32>,
    f: F,
) -> Buffer {
    let mut buffer = empty_buffer();
    for t in t_range {
        apply_shader(&mut buffer, t, &f);
        pcd8544.draw_buffer(&buffer);
    }
    buffer
}

pub fn run_optimized_mandelbrot(pcd8544: &mut Pcd8544) {
    let mut buffer = empty_buffer();
    for t in 0..32 {
        for col in 0..84 {
            let x = col as i32 - 42;
            let mut pixels = [false; 25];

            for y in 0..25 {
                pixels[y] = mandelbrot_zoom(x, y as i32, t);
            }

            for row in 0..3 {
                let base = 1 + (2 - row) * 8;
                let mut byte = 0;
                for bit in 0..8 {
                    byte += (pixels[base + bit] as u8) << (7 - bit);
                }
                buffer[6 * col + row] = byte;
            }
            for row in 3..6 {
                let base = (row - 3) * 8;
                let mut byte = 0;
                for bit in 0..8 {
                    byte += (pixels[base + bit] as u8) << bit;
                }
                buffer[6 * col + row] = byte;
            }
        }
        pcd8544.draw_buffer(&buffer);
    }
}

pub fn mandelbrot_zoom(px: i32, py: i32, t: i32) -> bool {
    let max_t = 32;
    if t >= max_t {
        return true;
    }

    let zoom: i32 = max_t - t;
    let cx = zoom * px / 2 - 200;
    let cy = zoom * py / 2;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for _ in 0..15 {
        if (x * x + y * y) > 4 << 16 {
            return false;
        }

        let xtemp = (x * x - y * y) / 256 + cx;
        y = (2 * x * y) / 256 + cy;
        x = xtemp;
    }
    true
}

pub fn deform_1_z(px: i32, py: i32, t: i32) -> bool {
    let r2 =
        1 + ((0x1400000 + sin(90 + t * 4)).wrapping_shr(13) * (px * px + py * py)).wrapping_shr(8);

    let x = px.wrapping_shl(16) / r2 + t.wrapping_shl(2);
    let y = py.wrapping_shl(16) / r2 + t.wrapping_shl(2);

    if (x & 0x80) + (y & 0x80) & 0x80 > 0 {
        return true;
    }
    false
}

fn sin(deg: i32) -> i32 {
    match deg {
        0...90 => sine_lut[deg as usize],
        91...180 => sine_lut[(180 - deg) as usize],
        181...270 => -sine_lut[(deg - 180) as usize],
        271...360 => -sine_lut[(360 - deg) as usize],
        _ => sin(deg % 360),
    }
}

// lookup of sine(degrees) << 24
static sine_lut: [i32; 91] = [
    0, 292802, 585516, 878051, 1170319, 1462230, 1753696, 2044628, 2334937, 2624534, 2913332,
    3201243, 3488179, 3774052, 4058775, 4342263, 4624427, 4905183, 5184444, 5462127, 5738145,
    6012416, 6284855, 6555380, 6823908, 7090357, 7354647, 7616696, 7876425, 8133755, 8388607,
    8640905, 8890569, 9137526, 9381700, 9623015, 9861400, 10096780, 10329085, 10558244, 10784186,
    11006844, 11226148, 11442033, 11654433, 11863283, 12068519, 12270079, 12467901, 12661925,
    12852093, 13038345, 13220626, 13398880, 13573052, 13743090, 13908942, 14070557, 14227886,
    14380880, 14529495, 14673683, 14813402, 14948608, 15079261, 15205321, 15326749, 15443508,
    15555563, 15662880, 15765426, 15863169, 15956080, 16044131, 16127295, 16205546, 16278860,
    16347217, 16410593, 16468971, 16522332, 16570660, 16613941, 16652161, 16685308, 16713373,
    16736347, 16754223, 16766995, 16774660, 16777216,
];
