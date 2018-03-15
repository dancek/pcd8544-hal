use Pcd8544;

pub fn rust_logo() -> [u8; 504] {
    let logo = include_bytes!("logo.bin");
    *logo
}

pub fn demo(pcd8544: &mut Pcd8544) {
    loop {
        let mut buffer = rust_logo();
        pcd8544.draw_buffer(&buffer);
        for i in 0..84 {
            for j in 0..6 {
                buffer[6*i+j] = 0xFF;
            }
            pcd8544.draw_buffer(&buffer);
        }
        for i in (0..84).rev() {
            for j in 0..6 {
                buffer[6*i+j] = 0x00;
            }
            pcd8544.draw_buffer(&buffer);
        }
    }
}
