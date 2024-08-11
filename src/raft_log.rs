use uuid::Uuid;

use crate::RaftNode;
use std::convert::{TryFrom, TryInto};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::os::unix::prelude::FileExt;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

const LOG_ENTRY_BYTE_LIMIT: usize = 512;


pub struct PageCache {
    pub backing_file: std::fs::File,
    
    page_cache: std::collections::HashMap<u64, [u8; LOG_ENTRY_BYTE_LIMIT]>
    page_cache_size: usize,

    buffer: Vec<u8>,
    buffer_start_address: Option<u64>,
    buffer_offset: Option<u64>,
}

pub struct PageCacheIO<'a> {
    disk_offset: usize,
    page_cache: &'a mut PageCache
}

impl PageCache {
    fn new() -> PageCache {
        let mut page_cache = std::collections::HashMap::new();
        let nfile = std::fs::File::create(format!("{}.cache", Uuid::new_v4())).unwrap();
        
        PageCache {
            backing_file: nfile,
            page_cache,
            page_cache_size: 0,
            buffer: vec![],
            buffer_start_address: None,
            buffer_offset: None
        }
    }

    fn write(&self) {

    }

    fn read(&self) {

    }

    fn sync(&mut self) {

    } 
}



#[derive(Debug, Clone)]
pub struct LogEntry {
    command: Vec<u8>,
    index: u64,
    term: u64,
    client_id: u64
}

impl LogEntry {
    pub fn apply_to_state_machine(&self, _node: RaftNode) {
        todo!("Finish this later");
    }

    // i.e. if we can fit our commands in a single page on disk
    fn command_length(comm_len: usize) -> usize{
        if (comm_len > LOG_ENTRY_BYTE_LIMIT - 53) {
            return LOG_ENTRY_BYTE_LIMIT - 53;
        } 
        comm_len 
    }

    fn store_log_metadata(&self, buffer: &mut [u8; LOG_ENTRY_BYTE_LIMIT as usize]) -> usize {
        *buffer = [0; LOG_ENTRY_BYTE_LIMIT as usize];
        let command_length: usize = self.command.len();

        buffer[0] = 1; // Entry start marker.
        buffer[5..13].copy_from_slice(&self.term.to_le_bytes());
        buffer[13..21].copy_from_slice(&self.index.to_le_bytes());
        buffer[37..45].copy_from_slice(&self.client_id.to_le_bytes());
        buffer[45..53].copy_from_slice(&command_length.to_le_bytes());

        let checksum = crc32c::crc32c(&buffer[5..53]);
        buffer[1..5].copy_from_slice(&checksum.to_le_bytes());

        let command_first_page = LogEntry::command_length(command_length);
        buffer[53..53 + command_first_page].copy_from_slice(&self.command[0..command_first_page]);
        command_length - command_first_page
    }

    fn store_overflow(&self, buffer: &mut [u8; LOG_ENTRY_BYTE_LIMIT as usize], offset: usize) -> usize {
        let to_write = self.command.len() - offset;
        let filled = if to_write > LOG_ENTRY_BYTE_LIMIT as usize - 1 {
            // -1 for the overflow marker.
            LOG_ENTRY_BYTE_LIMIT as usize - 1
        } else {
            to_write
        };
        buffer[0] = 0; // Overflow marker.
        buffer[1..1 + filled].copy_from_slice(&self.command[offset..offset + filled]);
        filled
    }


    fn encode(&self, buffer: &mut [u8; LOG_ENTRY_BYTE_LIMIT as usize], mut writer: impl std::io::Write) -> u64 {
        let to_write = self.store_log_metadata(buffer);
        writer.write_all(buffer).unwrap();
        let mut pages = 1;

        let mut written = self.command.len() - to_write;

        while written < self.command.len() {
            let filled = self.store_overflow(buffer, written);
            writer.write_all(buffer).unwrap();
            written += filled;
            pages += 1;
        }

        pages
    }

    fn recover_metadata(page: &[u8; LOG_ENTRY_BYTE_LIMIT as usize]) -> (LogEntry, u32, usize) {
        assert_eq!(page[0], 1); // Start of entry marker.
        let term = u64::from_le_bytes(page[5..13].try_into().unwrap());
        let index = u64::from_le_bytes(page[13..21].try_into().unwrap());
        let client_id = u64::from_le_bytes(page[37..53].try_into().unwrap());
        let command_length = u64::from_le_bytes(page[53..61].try_into().unwrap()) as usize;
        let stored_checksum = u32::from_le_bytes(page[1..5].try_into().unwrap());

        // recover_metadata() will only decode the first page's worth of
        // the command. Call recover_overflow() to decode any
        // additional pages.
        let command_first_page = LogEntry::command_length(command_length);
        let mut command = vec![0; command_length];
        command[0..command_first_page].copy_from_slice(&page[61..61 + command_first_page]);

        (
            LogEntry {
                index,
                term,
                command,
                client_id,
            },
            stored_checksum,
            command_first_page,
        )
    }

    fn recover_overflow(
        page: &[u8; LOG_ENTRY_BYTE_LIMIT as usize],
        command: &mut [u8],
        command_read: usize,
    ) -> usize {
        let to_read = command.len() - command_read;

        // Entry start marker is false for overflow page.
        assert_eq!(page[0], 0);

        let fill = if to_read > LOG_ENTRY_BYTE_LIMIT as usize - 1 {
            // -1 for the entry start marker.
            LOG_ENTRY_BYTE_LIMIT as usize - 1
        } else {
            to_read
        };
        command[command_read..command_read + fill].copy_from_slice(&page[1..1 + fill]);
        fill
    }


    fn decode(mut reader: impl std::io::Read) -> LogEntry {
        let mut page = [0; LOG_ENTRY_BYTE_LIMIT as usize];
        // Since entries are always encoded into complete PAGESIZE
        // bytes, for network or for disk, it should always be
        // reasonable to block on an entire PAGESIZE of bytes, for
        // network or for disk.
        reader.read_exact(&mut page).unwrap();

        let (mut entry, stored_checksum, command_read) = LogEntry::recover_metadata(&page);
        let mut actual_checksum = CRC32C::new();
        actual_checksum.update(&page[5..61]);

        let mut read = command_read;
        while read < entry.command.len() {
            reader.read_exact(&mut page).unwrap();
            let filled = LogEntry::recover_overflow(&page, &mut entry.command, read);
            read += filled;
        }

        actual_checksum.update(&entry.command);
        assert_eq!(stored_checksum, actual_checksum.sum());
        entry
    }

    fn decode_from_pagecache(pagecache: &mut PageCache, offset: u64) -> (LogEntry, u64) {
        let mut reader = PageCacheIO { offset, pagecache };
        let entry = LogEntry::decode(&mut reader);
        let offset = reader.disk_offset;

        (entry, offset)
    }

    pub fn decode_from_bytes(&mut self) -> Vec<u8> {
        todo!("Need to write decode logic");
    }
}

type Log = Vec<LogEntry>;
