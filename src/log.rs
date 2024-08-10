use crate::RaftNode;

pub struct LogEntry {
    command: Vec<u8>,
    term: i64,
}

impl LogEntry {
    pub fn apply_to_state_machine(&self, node: RaftNode) {

    }
}

pub type Log = Vec<LogEntry>;

