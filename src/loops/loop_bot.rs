use super::super::*;

pub fn run_loop_bot(
    receiver: std::sync::mpsc::Receiver<String>,
    sender: std::sync::mpsc::Sender<websocket::OwnedMessage>,
) {
    let mut running = true;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        if running {
            log_debug!("test");
        }

        let message = match receiver.try_recv() {
            Ok(m) => m,
            Err(_) => "/nothing".to_string(),
        };

        match message.as_str() {
            "" => {
                if running {
                    log_debug!("loop_bot message pause");
                } else {
                    log_debug!("loop_bot message unpause");
                }

                running = !running;
            }
            "/close" => {
                log_debug!("loop_bot message close");
                break;
            }
            "/nothing" => {}
            _ => {
                log_debug!("loop_bot message unknown: {}", message);
            }
        };
    }

    log_info!("Exiting loop_bot...");
}
