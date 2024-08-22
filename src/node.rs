use futures::prelude::*;
use std::collections::HashMap;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};
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

        }
    }

    pub fn 
}
