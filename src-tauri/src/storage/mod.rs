pub struct JsonStorage<T> {
    path: String,
}

impl<T> JsonStorage {
    pub fn new(path: String) -> JsonStorage {
        JsonStorage { path }
    }

    /**
     * 获取存储信息
     */
    pub fn getStorageInfo(&self) -> T {}

    /**
     * 保存配置
     */
    pub fn saveStorage(&self, config: Vec<T>) {}
}
