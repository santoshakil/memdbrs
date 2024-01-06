use anyhow::Result;

pub fn init() -> Result<()> {
    if let Err(err) = logrs::init_logrs("./logrs.log".to_owned()) {
        log::error!("Failed to initialize logrs: {}", err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
