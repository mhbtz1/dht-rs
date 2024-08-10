An implementation of a distributed hash table in Rust using RAFT for distributed consensus. 
For proof-of-concept implementation, RPCs are simulated by writing RPC arguments to disk, and having replies be made by reading 
the requests stored on the disk and saving the reply to disk.