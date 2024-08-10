use futures::prelude::*;
use tarpc::{
    client, context,
    server::{self, Channel},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver} };

enum NodeState {
    Candidate,
    Leader,
    Follower
}

#[derive(Debug)]
pub struct LogObject;

pub struct RaftCluster {
    pub cluster_mutex: Arc<Mutex<usize>>,
    pub raft_nodes: Vec<RaftNode>
}

#[derive(Debug)]
pub struct RaftNode {
    pub node_mutex: Arc<Mutex<usize>>,
    //senders to all other local raft nodes
    pub senders: HashMap<usize, Sender<()> >,
    pub receivers: HashMap<usize, Receiver<()> >,

    pub store: HashMap<i64, i64>,
    pub log: Vec<LogObject>,


    pub is_leader: bool,
    pub current_term: usize,
    pub voted_for: i64,
    pub next_index: Vec<i64>,
    pub match_index: Vec<i64>,

    pub commit_index: i64,
    pub last_applied: i64
}


impl RaftNode {
    pub fn append_entries() {

    }

    pub fn request_vote() {

    }

}