use anyhow::Result;

pub fn open(path: String) -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    let len = file.metadata()?.len();
    log::info!("File size: {}", len);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        crate::init::init_log().unwrap();
        assert!(open("./default.memdbrs".to_owned()).is_ok());
    }
}
