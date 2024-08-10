mod node;
mod rpc;
mod log;

use node::{RaftNode, RaftCluster};
use rpc::{RequestVoteReq, RequestVoteReply, AppendEntriesReq, AppendEntriesReply};
use log::{Log, LogEntry};

fn main() {
    println!("Hello, world!");
}
