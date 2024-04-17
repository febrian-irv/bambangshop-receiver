use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::{self, Notification};

// Singleton of Databases
lazy_static! {
    static ref NOTIFICATION: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {
    pub fn add (notification: Notification) -> Notification {
        NOTIFICATION.write().unwrap().push(notification.clone());
        return notification;
    }
}