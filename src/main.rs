mod node;
mod rpc;
mod raft_log;

use node::{RaftNode, RaftCluster};
use rpc::{RequestVoteReq, RequestVoteReply, AppendEntriesReq, AppendEntriesReply};
use raft_log::{Log, LogEntry};

fn main() {
    println!("Hello, world!");
}
