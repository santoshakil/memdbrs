use std::os::fd::AsRawFd;

use anyhow::Result;

pub fn open(path: String) -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    file.set_len(1024 * 1024 * 1024)?;
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

    log::info!("Start address: {:p}", memory);
    let end_address = (memory as usize) + len as usize;
    log::info!("End address: {:p}", end_address as *const u8);

    Ok(())
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
