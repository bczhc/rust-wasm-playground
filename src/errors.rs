pub trait AnyhowExt<T> {
    fn map_err_string(self) -> Result<T, String>;
}

impl<T> AnyhowExt<T> for anyhow::Result<T> {
    fn map_err_string(self) -> Result<T, String> {
        self.map_err(|e| format!("{e}"))
    }
}

