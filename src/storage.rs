
use model::*;
use slab::Slab;
use yew::services::storage::{StorageService, Area};
use failure::Error;

const BASE_KEY: &'static str = "taskigt.storage";

pub struct LocalDocumentStorage {
    storage_service: StorageService
}

impl LocalDocumentStorage {
    pub fn new() -> Self {
        LocalDocumentStorage {
            storage_service: StorageService::new(Area::Local)
        }
    }

    pub fn save(&mut self, title: &str, content: String) {
        let mut key = BASE_KEY.to_string();
        key.push('.');
        key.push_str(title);
        self.storage_service.store(&key, Ok(content));
    }

    pub fn restore(&mut self, title: String) -> Result<String, Error> {
        let mut key = BASE_KEY.to_string();
        key.push('.');
        key.push_str(&title);
        self.storage_service.restore(&key)
    }
}

