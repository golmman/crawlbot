use crate::model::cws::msg::CwsMsg;
use crate::loops::bot_loop::BotLoopState;
use crate::model::instruction::{CrawlScript, Instruction};
use crate::routines::{
    supply_routine_abandon, supply_routine_descend, supply_routine_explore, supply_routine_fight,
    supply_routine_idle10, supply_routine_idle5, supply_routine_main, supply_routine_pick_mifi,
    supply_routine_pick_trbe, supply_routine_start, supply_routine_step_east,
    supply_routine_step_none, supply_routine_step_north, supply_routine_step_north_east,
    supply_routine_step_north_west, supply_routine_step_south, supply_routine_step_south_east,
    supply_routine_step_south_west, supply_routine_step_west,
};
use crate::{log_crawl, log_debug};

impl BotLoopState {
    pub fn abandon(&mut self) {
        self.enqueue_primary(supply_routine_abandon);
    }

    pub fn clear_routines(&mut self) {
        self.secondary_queue.clear();
    }

    pub fn close(&mut self) {
        let _ = self.sender_websocket.send(Instruction::Close);
        self.exit_loop = true;
    }

    pub fn crawl_message(&mut self, message: CwsMsg) {
        if let Some(message_type) = &message.msg {
            match message_type.as_str() {
                "map" => self.game_state.update_map(message),
                "input_mode" => self.game_state.update_input_mode(message),
                "msgs" => self.game_state.update_game_state_with_msgs(message),
                "ping" => self.pong(),
                _ => {}
            };
        }
    }

    pub fn crawl_output(&mut self, data: String) {
        let _ = self.sender_websocket.send(Instruction::CrawlOutput(data));
    }

    pub fn descend(&mut self) {
        self.enqueue_primary(supply_routine_descend);
    }

    pub fn explore(&mut self) {
        self.enqueue_primary(supply_routine_explore);
    }

    pub fn fight(&mut self) {
        self.enqueue_primary(supply_routine_fight);
    }

    pub fn get_status(&mut self) {
        log_debug!("----- STATUS -----");
        println!("secondary_queue = {:#?}", self.secondary_queue);
        println!("game_state = {:#?}", self.game_state);
        log_debug!("--- END STATUS ---");
    }

    pub fn idle10(&mut self) {
        self.enqueue_secondary(supply_routine_idle10);
    }

    pub fn idle5(&mut self) {
        self.enqueue_secondary(supply_routine_idle5);
    }

    pub fn main(&mut self) {
        self.enqueue_secondary(supply_routine_main);
    }

    pub fn map(&mut self) {
        log_debug!("----- MAP -----");
        self.game_state.print_map();
        log_debug!("--- END MAP ---");
    }

    pub fn nothing(&mut self) {}

    pub fn pause(&mut self) {
        self.game_state.pause();
    }

    pub fn pick_mifi(&mut self) {
        self.enqueue_primary(supply_routine_pick_mifi);
    }

    pub fn pick_trbe(&mut self) {
        self.enqueue_primary(supply_routine_pick_trbe);
    }

    pub fn ping(&mut self, data: Vec<u8>) {
        let _ = self.sender_websocket.send(Instruction::Pong(data));
    }

    pub fn script(&mut self, crawl_script: CrawlScript) {
        self.secondary_queue
            .append(&mut crawl_script.evaluate(&self.game_state))
        // self.enqueue_secondary(|| crawl_script.evaluate(&self.game_state))
    }

    pub fn start(&mut self) {
        self.enqueue_primary(supply_routine_start);
    }

    pub fn step_south_west(&mut self) {
        self.enqueue_primary(supply_routine_step_south_west);
    }

    pub fn step_south(&mut self) {
        self.enqueue_primary(supply_routine_step_south);
    }

    pub fn step_south_east(&mut self) {
        self.enqueue_primary(supply_routine_step_south_east);
    }

    pub fn step_west(&mut self) {
        self.enqueue_primary(supply_routine_step_west);
    }

    pub fn step_none(&mut self) {
        self.enqueue_primary(supply_routine_step_none);
    }

    pub fn step_east(&mut self) {
        self.enqueue_primary(supply_routine_step_east);
    }

    pub fn step_north_west(&mut self) {
        self.enqueue_primary(supply_routine_step_north_west);
    }

    pub fn step_north(&mut self) {
        self.enqueue_primary(supply_routine_step_north);
    }

    pub fn step_north_east(&mut self) {
        self.enqueue_primary(supply_routine_step_north_east);
    }

    pub fn unpause(&mut self) {
        self.game_state.unpause();
    }
}
