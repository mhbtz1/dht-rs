use crate::RaftNode;

#[derive(Debug, Clone)]
pub struct LogEntry {
    command: Vec<u8>,
    term: i64,
}

impl LogEntry {
    pub fn apply_to_state_machine(&self, node: RaftNode) {
        todo!("Finish this later");
    }
    pub fn convert_to_bytes(&mut self) -> Vec<u8> {
        let mut log_vec = vec![];
        let mut command = self.command.clone();
        let signal = crc32c::crc32c(&self.command.as_slice());
        log_vec.append(&mut command);
        log_vec.append(&mut self.term.to_le_bytes().to_vec());
        log_vec.append(&mut signal.to_le_bytes().to_vec());
        log_vec
    }

    pub fn decode_from_bytes(&mut self) -> Vec<u8> {
        
    }
}

pub struct Log {
    log: Vec<LogEntry>
}

impl Log {
    pub fn convert_to_bytes(&mut self) -> Vec<u8> {
        let mut total_vec = vec![];
        for mut l in self.log.clone().into_iter() {
            total_vec.append(&mut l.convert_to_bytes());
        }
        total_vec
    }

    pub fn decode_from_byted(&mut self) -> [u8] {

    }
}

