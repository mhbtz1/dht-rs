use futures::prelude::*;
use tarpc::{
    client, context,
    server::{self, Channel},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver} };

type FunctionCall = Box<dyn FnOnce() + Send + 'static>;

pub struct RaftCluster<K, V> {
    pub cluster_mutex: Arc<Mutex<usize>>,
    pub raft_nodes: Vec<RaftNode<K, V>>
}

#[derive(Debug)]
pub struct RaftNode<K, V> {
    node_mutex: Arc<Mutex<usize>>,
    //senders to all other local raft nodes
    senders: HashMap<usize, Sender<()> >,
    receivers: HashMap<usize, Receiver<()> >,

    store: HashMap<K, V>,
    log: Vec<V>,


    is_leader: bool,
    current_term: usize,
    voted_for: i64,
    next_index: Vec<V>,
    match_index: Vec<V>,

    commit_index: i64,
    last_applied: i64

}

impl<K, V> RaftNode<K, V> {
    pub fn new(senders: HashMap<usize, Sender<()>>, receivers: HashMap<usize, Receiver<()>>) -> Self {
        RaftNode { 
            node_mutex: Arc::new(Mutex::new(0)),
            senders,
            receivers,
            store: HashMap::new(),
            log: vec![],
            is_leader: false,
            current_term: 0,
            voted_for: -1,
            next_index: vec![],
            match_index: vec![],
            commit_index: 0,
            last_applied: 0
        }
    }
    
    pub fn append_entries(&mut self) {

    }

    pub fn request_vote(&mut self) {
    
    }
}


