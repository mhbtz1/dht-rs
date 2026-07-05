use futures::prelude::*;
use std::collections::HashMap;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use crate::rpc::{AppendEntriesReply, AppendEntriesReq};

use tarpc::{
    client, context,
    server::{self, Channel},
};

enum NodeState {
    Candidate,
    Leader,
    Follower,
}

#[derive(Debug)]
pub struct LogObject;

pub struct RaftCluster {
    pub cluster_mutex: Arc<Mutex<usize>>,
    pub raft_nodes: Vec<RaftNode>,
}

#[derive(Debug)]
pub struct RaftNode {
    pub node_mutex: Mutex<usize>,
    //senders to all other local raft nodes
    pub senders: HashMap<usize, Sender<()> >,
    pub receivers: HashMap<usize, Sender<()> >,

    pub store: HashMap<i64, i64>,
    pub log: Vec<LogObject>,

    pub is_leader: bool,
    pub current_term: usize,
    pub voted_for: i64,
    pub next_index: Vec<i64>,
    pub match_index: Vec<i64>,

    pub commit_index: i64,
    pub last_applied: i64,
}

impl RaftNode {

    pub fn new() -> RaftNode {
        RaftNode {
            node_mutex: Mutex::new(0),
            senders: HashMap::new(),
            receivers: HashMap::new(),

            store: HashMap::new(),
            log: Vec::new(),

            is_leader: false,
            current_term: 0,
            voted_for: -1, // -1 sentinel: hasn't voted for anyone this term.
            next_index: Vec::new(),
            match_index: Vec::new(),

            commit_index: 0,
            last_applied: 0,
        }
    }

    // Follower-side handler for the leader's AppendEntries RPC.
    pub fn append_entries(&mut self, request: AppendEntriesReq) -> AppendEntriesReply {
        if request.term < self.current_term as u64 {
            return AppendEntriesReply {
                term: self.current_term as u64,
                success: false,
            };
        }

        self.current_term = request.term as usize;
        self.is_leader = false;
        self.voted_for = -1;

        if request.leader_commit as i64 > self.commit_index {
            self.commit_index = request.leader_commit as i64;
        }

        AppendEntriesReply {
            term: self.current_term as u64,
            success: true,
        }
    }
}
