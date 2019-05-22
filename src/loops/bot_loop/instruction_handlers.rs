use crate::loops::bot_loop::BotLoopState;
use crate::model::instruction::{Instruction, Routine};
use crate::model::GameState;
use crate::routines::{
    supply_routine_abandon, supply_routine_idle10, supply_routine_idle5, supply_routine_main,
    supply_routine_pick_mifi, supply_routine_pick_trbe, supply_routine_start,
};
use crate::{log_crawl, log_debug};
use serde_json::Value;

impl BotLoopState {
    pub fn abandon(&mut self) {
        self.enqueue_routine(supply_routine_abandon);
    }

    pub fn clear_routines(&mut self) {
        self.secondary_queue.clear();
    }

    pub fn close(&mut self) {
        let _ = self.sender_websocket.send(Instruction::Close);
        self.exit_loop = true;
    }

    pub fn crawl_input(&mut self, crawl_message: Value) {
        let crawl_msg = &crawl_message["msg"];

        match crawl_msg.as_str().unwrap() {
            "ping" => self.pong(),
            "msgs" => self.update_game_state_with_msgs(crawl_message),
            "input_mode" => self.update_input_mode(crawl_message),
            _ => {}
        }
    }

    pub fn crawl_output(&mut self, data: String) {
        let _ = self.sender_websocket.send(Instruction::CrawlOutput(data));
    }

    pub fn get_status(&mut self) {
        log_debug!("--- STATUS ---");
        println!("secondary_queue = {:#?}", self.secondary_queue);
        println!("game_state = {:#?}", self.game_state);
        log_debug!("--------------");
    }
    pub fn idle10(&mut self) {
        self.enqueue_routine(supply_routine_idle10);
    }
    pub fn idle5(&mut self) {
        self.enqueue_routine(supply_routine_idle5);
    }
    pub fn if_then_else(
        &mut self,
        check: fn(GameState) -> bool,
        then_routine_supplier: fn() -> Routine,
        else_routine_supplier: fn() -> Routine,
    ) {
        if check(self.game_state) {
            self.enqueue_routine(then_routine_supplier);
        } else {
            self.enqueue_routine(else_routine_supplier);
        }
    }
    pub fn main(&mut self) {
        self.enqueue_routine(supply_routine_main);
    }
    pub fn nothing(&mut self) {}
    pub fn pause(&mut self) {
        self.game_state.pause();
    }
    pub fn pick_mifi(&mut self) {
        self.enqueue_routine(supply_routine_pick_mifi);
    }
    pub fn pick_trbe(&mut self) {
        self.enqueue_routine(supply_routine_pick_trbe);
    }
    pub fn ping(&mut self, data: Vec<u8>) {
        let _ = self.sender_websocket.send(Instruction::Pong(data));
    }
    pub fn script(&mut self, evaluate: fn(GameState) -> Routine) {
        self.secondary_queue.append(&mut evaluate(self.game_state))
    }
    pub fn start(&mut self) {
        self.enqueue_routine(supply_routine_start);
    }
    pub fn unpause(&mut self) {
        self.game_state.unpause();
    }
}
