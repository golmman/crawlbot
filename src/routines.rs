use std::collections::VecDeque;
use crate::loops::internal_message::InternalMessage;

pub const NOTHING5: &[InternalMessage] = &[
    InternalMessage::Nothing,
    InternalMessage::Nothing,
    InternalMessage::Nothing,
    InternalMessage::Nothing,
    InternalMessage::Nothing,
];

pub fn push_routine(queue: &mut VecDeque<InternalMessage>, routine: &[InternalMessage]) {
    for r in routine {
        queue.push_back(r.clone());
    }
}
