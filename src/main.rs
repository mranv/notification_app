extern crate notify_rust;
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // Send notification
        send_notification("Hello from Rust!");

        // Sleep for one second
        thread::sleep(Duration::from_secs(1));
    }
}

fn send_notification(message: &str) {
    // Create a new notification
    let notification = Notification::new()
        .summary("Rust Notification")
        .body(message)
        .icon("dialog-information")
        .finalize(); // Use finalize to construct the Notification object

    // Check if the notification is supported
    if let Err(error) = notification.show() {
        eprintln!("Error showing notification: {:?}", error);
    }
}
