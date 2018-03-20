use Pcd8544;
use fpa::I8F24;

type Buffer = [u8; 504];

static RUST_LOGO: &'static Buffer = include_bytes!("logo.bin");

fn empty_buffer() -> Buffer {
    [0u8; 504]
}

pub fn demo(pcd8544: &mut Pcd8544) {
    loop {
        run_shader(pcd8544, 0..30, |x,y,t| (x*6).pow(2) + (y*7).pow(2) < (t*6).pow(2));

        for _ in 0..20 {
            pcd8544.draw_buffer(RUST_LOGO);
        }

        run_shader(pcd8544, 0..32, mandelbrot_zoom_int);
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
            buffer[6*col + row] = byte;
        }
    }
}

pub fn run_shader<F: Fn(i32, i32, i32) -> bool>(pcd8544: &mut Pcd8544, t_range: ::core::ops::Range<i32>, f: F) -> Buffer {
    let mut buffer = empty_buffer();
    for t in t_range {
        apply_shader(&mut buffer, t, &f);
        pcd8544.draw_buffer(&buffer);
    }
    buffer
}

pub fn mandelbrot_zoom(px: i32, py: i32, t: i32) -> bool {
    let max_t = 10;
    if t >= max_t {
        return false;
    }

    let zoom = (I8F24(max_t - t).unwrap()) / I8F24(8i8);
    let cx = (I8F24(-7i8) / I8F24(5i8)) + zoom * I8F24(px).unwrap() / I8F24(24i8);
    let cy = zoom * I8F24(py).unwrap() / I8F24(24i8);

    let mut x = I8F24(0i8);
    let mut y = I8F24(0i8);
    for _ in 0..10 {
        if (x*x + y*y) > I8F24(4i8) {
            return false;
        }

        let xtemp = x*x - y*y + cx;
        y = 2 * x * y + cy;
        x = xtemp;
    }
    true
}

pub fn mandelbrot_zoom_int(px: i32, py: i32, t: i32) -> bool {
    let max_t = 32;
    if t >= max_t {
        return true;
    }

    let zoom: i32 = (max_t - t);
    let cx = zoom * px / 2 - 200;
    let cy = zoom * py / 2;

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xtemp: i32 = 0;

    for _ in 0..15 {
        if (x*x + y*y) > 4<<16 {
            return false;
        }

        xtemp = (x*x - y*y) / 256 + cx;
        y = (2*x*y) / 256 + cy;
        x = xtemp;
    }
    true
}
