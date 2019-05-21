use super::bot_loop::BotLoopState;
use super::super::routines::*;
use super::super::model::instruction::Instruction;

impl BotLoopState {
    pub fn abandon(&mut self) {
        self.enqueue_routine(supply_routine_abandon);
    }

    pub fn close(&mut self) {
        let _ = self.sender_websocket.send(Instruction::Close);
        self.exit_loop = true;
    }
}