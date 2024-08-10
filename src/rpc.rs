use std::io::{Read, Write, BufReader, BufWriter};
use crate::Log;

pub struct RequestVoteReply {
    term: i64,
    vote_granted: bool
}

pub struct AppendEntriesReply {
    term: i64,
    success: bool
}

pub struct RequestVoteReq  {
    term: i64,
    candidate_id: i64,
    last_log_index: usize,
    last_log_term: i64,
    entries: Log
}

pub struct AppendEntriesReq {
    term: i64,
    leade_id: i64, 
    prev_log_index: usize,
    prev_log_term: i64
}

// in order to simplify the application of RPC for this example, we will use the disk as a proxy for performing RPCs between Raft nodes
struct RPCMessageEncoder<T: std::io::Write> {
    request_id: u64,
    sender_id: u64,
    writer: BufWriter<T>,
    written_to_disk: Vec<u8>
}

impl <T: std::io::Write> RPCMessageEncoder<T> {
    fn new(request_id: u64, sender_id: u64, writer: BufWriter<T>) -> RPCMessageEncoder<T> {
        RPCMessageEncoder {
            request_id,
            writer,
            written_to_disk: vec![],
            sender_id,
        }
    }


    fn metadata(&mut self, kind: u8, term: u64) {
        self.written_to_disk.extend_from_slice(&self.request_id.to_le_bytes());
        
        self.written_to_disk.extend_from_slice(&self.sender_id.to_le_bytes());

        self.written_to_disk.push(kind);

        self.written_to_disk.extend_from_slice(&term.to_le_bytes());
        
        self.writer.write_all(&self.written_to_disk).unwrap();
    }
    
}
