mod node;
mod raft_log;
mod rpc;

use node::{RaftCluster, RaftNode};
use raft_log::{Log, LogEntry};
use rpc::{AppendEntriesReply, AppendEntriesReq, RequestVoteReply, RequestVoteReq};

fn main() {
    println!("Hello, world!");

}
