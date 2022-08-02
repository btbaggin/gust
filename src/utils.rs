#[macro_export]
macro_rules! unsafe_read_type {
    ($ty:ty, $file:expr, $index:expr) => {{
        let mut config: $ty = unsafe { std::mem::zeroed() };
        let config_size = std::mem::size_of::<$ty>();
        unsafe {
            use std::io::Read;
            let config_slice = std::slice::from_raw_parts_mut(&mut config as *mut _ as *mut u8, config_size);
            let mut data_slice = &$file[$index..];
            data_slice.read_exact(config_slice).unwrap();
        }
        $index += config_size;
        config
            // let size = std::mem::size_of::<$ty>();
            // let value = <$ty>::from_le_bytes($file[$index..($index + size)].try_into().unwrap());
            // $index += size;
            // value
        }};
}
pub use unsafe_read_type;