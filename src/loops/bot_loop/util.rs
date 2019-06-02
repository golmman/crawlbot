use crate::loops::bot_loop::BotLoopState;
use crate::model::instruction::Instruction;
use crate::model::instruction::Routine;
use crate::{log_crawl, log_debug};

impl BotLoopState {
    pub fn enqueue_primary(&mut self, routine_supplier: fn() -> Routine) {
        self.primary_queue.append(&mut routine_supplier())
    }

    pub fn enqueue_instruction(&mut self, instruction_supplier: fn() -> Instruction) {
        self.primary_queue.push_back(instruction_supplier())
    }

    pub fn enqueue_secondary(&mut self, routine_supplier: fn() -> Routine) {
        self.secondary_queue.append(&mut routine_supplier())
    }

    pub fn pong(&self) {
        let _ = self
            .sender_websocket
            .send(Instruction::CrawlOutput("{\"msg\":\"pong\"}".to_string()));
    }

    pub fn fill_primary_queue(&mut self) {
        let message_stdin = self
            .receiver_stdin
            .try_recv()
            .unwrap_or(Instruction::Nothing);

        let message_websocket = self
            .receiver_websocket
            .try_recv()
            .unwrap_or(Instruction::Nothing);

        if message_stdin.is_something() {
            self.primary_queue.push_front(message_stdin);
        }

        if message_websocket.is_something() {
            self.primary_queue.push_front(message_websocket);
        }

        if !self.game_state.is_paused() {
            if self.primary_queue.is_empty() {
                log_debug!(
                    "primary_queue empty, idle_ticks: {}",
                    self.game_state.get_idle_ticks()
                );
                self.game_state.inc_idle_ticks();
                if self.game_state.get_idle_ticks() > 3 {
                    log_debug!("primary_queue empty, pausing.");
                    self.game_state.pause();
                }
            } else {
                self.game_state.set_idle_ticks(0);
            }

            if !self.secondary_queue.is_empty() {
                let routine_message = self.secondary_queue.pop_front().unwrap();
                log_debug!("Popping from routine queue: {:?}", routine_message);
                self.primary_queue.push_back(routine_message);
            }
        }
    }

    pub fn get_next_instruction(&mut self) -> Instruction {
        let message = self
            .primary_queue
            .pop_front()
            .unwrap_or(Instruction::Nothing);

        if message.is_something() {
            let m = format!("{:?}", message);
            log_debug!("Processing {:80.80} ... {} total chars", m, m.len());
        }

        message
    }
}
