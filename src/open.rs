use std::{io::Write, os::fd::AsRawFd};

use anyhow::Result;
use rand::Rng;

pub fn open(path: String) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    // writing 1 gb random data to the file if the len is 0 just for testing
    if file.metadata()?.len() == 0 {
        log::info!("Writing 1gb random data to the file");
        let mut rng = rand::thread_rng();
        let mut buffer = [0u8; 1024 * 1024];
        for _ in 0..1024 {
            rng.fill(&mut buffer[..]);
            file.write_all(&buffer[..])?;
        }
    }

    let len = file.metadata()?.len();
    log::info!("File size: {}mb", len / 1024 / 1024);

    let memory = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            len as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if memory == libc::MAP_FAILED {
        anyhow::bail!("Failed to mmap file");
    }

    // Printing the start and end address of the memory mapping
    log::info!("Start address: {:p}", memory);
    let end_address = (memory as usize) + len as usize;
    log::info!("End address: {:p}", end_address as *const u8);

    // Reading 10 bytes after a offset
    let offset = 1024 * 1024 * 1024 / 2;
    let bytes_to_read = 10;
    let data_slice = unsafe {
        std::slice::from_raw_parts(memory.offset(offset as isize) as *const u8, bytes_to_read)
    };
    log::info!("Data after offset {}: {:?}", offset, data_slice);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        crate::init::init_log().unwrap();
        match open("./default.memdbrs".to_owned()) {
            Ok(_) => (),
            Err(e) => panic!("Open failed with error: {}", e),
        }
    }
}
