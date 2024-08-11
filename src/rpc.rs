use crate::LogEntry;
use std::{
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use tempfile::NamedTempFile;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestVoteReply {
    term: u64,
    vote_granted: bool,
}

#[derive(Debug, Clone)]
pub struct AppendEntriesReply {
    term: u64,
    success: bool,
}

#[derive(Debug, Clone)]
pub struct RequestVoteReq {
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Debug, Clone)]
pub struct AppendEntriesReq {
    pub term: u64,
    pub leader_id: u64,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    pub entries: crate::Log,
    pub leader_commit: u64,
}
// in order to simplify the application of RPC for this example, we will use the disk as a proxy for performing RPCs between Raft nodes
/*
Disk format for RPC calls
1. RequestID (8 bytes)
2. SenderID (8 bytes)
3. Kind (of RPC call being made) (1 byte)
4. Term (of Raft Node) (8 bytes)
5. Any data specific to the RPC (AppendEntries vs RequestVote) (n bytes)
*/

struct RPCMessageEncoder<T: std::io::Write> {
    request_id: Uuid,
    sender_id: u64,
    writer: BufWriter<T>,
    written_to_disk: Vec<u8>,
}

impl<T: std::io::Write> RPCMessageEncoder<T> {
    fn new(request_id: Uuid, sender_id: u64, writer: BufWriter<T>) -> RPCMessageEncoder<T> {
        RPCMessageEncoder {
            request_id,
            writer,
            written_to_disk: vec![],
            sender_id,
        }
    }

    fn metadata(&mut self, kind: u8, term: u64) {
        self.written_to_disk
            .extend_from_slice(&self.request_id.to_bytes_le());

        self.written_to_disk
            .extend_from_slice(&self.sender_id.to_le_bytes());

        self.written_to_disk.push(kind);

        self.written_to_disk.extend_from_slice(&term.to_le_bytes());

        self.writer.write_all(&self.written_to_disk).unwrap();
    }

    // data from a generic RPC call (i.e. AppendEntries vs RequestVote )
    fn data(&mut self, data: &[u8]) {
        let offset = self.written_to_disk.len();
        self.written_to_disk.extend_from_slice(data);
        self.writer
            .write_all(&self.written_to_disk[offset..])
            .unwrap();
    }

    fn done(&mut self) {
        let checksum = crc32c::crc32c(&self.written_to_disk);

        self.writer.write_all(&checksum.to_le_bytes()).unwrap();
        self.writer.flush().unwrap();
    }
}

impl RequestVoteReq {
    fn send_rpc(&self, node_id: u64) -> anyhow::Result<()> {
        let tmp_file = NamedTempFile::new_in("/tmp")?;
        let writer = BufWriter::new(tmp_file);

        let mut rpc_encoder = RPCMessageEncoder::new(Uuid::new_v4(), node_id, writer);
        rpc_encoder.metadata(0, self.term);
        rpc_encoder.data(&self.candidate_id.to_le_bytes());
        rpc_encoder.data(&self.last_log_index.to_le_bytes());
        rpc_encoder.data(&self.last_log_term.to_le_bytes());
        rpc_encoder.done();

        Ok(())
    }

    fn decode<T: std::io::Read>(
        mut buf_reader: BufReader<T>,
        metadata: [u8; 33],
        request_id: u64,
        term: u64,
    ) -> std::result::Result<RequestVoteReq, String> {
        let mut buf: [u8; 64] = [0; 64];
        buf[0..metadata.len()].copy_from_slice(&metadata);
        //read_exact will mutate the buffer and append the other RPC specific information into it.
        let content = buf_reader.read_exact(&mut buf[metadata.len()..]).unwrap();
        let checksum = u32::from_le_bytes(buf[32..36].try_into().unwrap());
        if checksum != crc32c::crc32c(&buf[0..36]) {
            return Err("Error!".to_string());
        }

        let candidate_id = u64::from_le_bytes(buf[33..49].try_into().unwrap());
        let last_log_index = u64::from_le_bytes(buf[49..57].try_into().unwrap());
        let last_log_term = u64::from_le_bytes(buf[57..64].try_into().unwrap());

        Ok(RequestVoteReq {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        })
    }
}

impl AppendEntriesReq {
    fn send_rpc(&self, node_id: u64) -> anyhow::Result<()> {
        let tmp_file = NamedTempFile::new_in("/tmp")?;
        let writer = BufWriter::new(tmp_file);

        let mut rpc_encoder = RPCMessageEncoder::new(Uuid::new_v4(), node_id, writer);

        rpc_encoder.metadata(0, self.term);
        rpc_encoder.data(&self.leader_id.to_le_bytes());
        rpc_encoder.data(&self.prev_log_index.to_le_bytes());
        rpc_encoder.data(&self.prev_log_term.to_le_bytes());
        rpc_encoder.data(&self.leader_commit.to_le_bytes());
        rpc_encoder.done();

        Ok(())
    }
}
