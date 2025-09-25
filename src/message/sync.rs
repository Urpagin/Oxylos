use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use crate::message::hash::sha256_file;
use crate::storage::fsio::{copy_msg, list_files};

pub struct SyncReport {
    pub copied_to_remote: usize,
    pub copied_to_local: usize,
}

pub fn bidirectional_sync(
    local_outbox: &[PathBuf],
    remote_dirs: &[Vec<PathBuf>],
    local_dest: &str,
    remote_dest: &str,
) -> io::Result<SyncReport> {
    let mut own_map = HashMap::new();
    for f in local_outbox {
        let digest = sha256_file(f)?;
        own_map.insert(digest, f);
    }
    let mut remote_map = HashMap::new();
    for group in remote_dirs {
        for f in group {
            let digest = sha256_file(f)?;
            remote_map.insert(digest, f);
        }
    }

    let commons: Vec<[u8; 32]> = own_map
        .keys()
        .filter(|k| remote_map.contains_key(*k))
        .copied()
        .collect();
    for k in commons {
        own_map.remove(&k);
        remote_map.remove(&k);
    }

    let mut copied_to_remote = 0usize;
    for msg in own_map.values() {
        let name = uuid::Uuid::new_v4().to_string();
        copy_msg(msg.to_str().unwrap(), &format!("{remote_dest}{name}"))?;
        copied_to_remote += 1;
    }

    let mut copied_to_local = 0usize;
    for msg in remote_map.values() {
        let name = uuid::Uuid::new_v4().to_string();
        copy_msg(msg.to_str().unwrap(), &format!("{local_dest}{name}"))?;
        copied_to_local += 1;
    }

    Ok(SyncReport {
        copied_to_remote,
        copied_to_local,
    })
}

pub fn auto_sync() -> io::Result<SyncReport> {
    let local = list_files(crate::config::DEFAULT_OUT_MSG)?;
    let remote_p = list_files(crate::storage::paths::persistent_dir())?;
    let remote_r = list_files(crate::storage::paths::ram_dir())?;
    bidirectional_sync(
        &local,
        &[remote_p, remote_r],
        crate::config::DEFAULT_OUT_MSG,
        crate::storage::paths::persistent_dir(),
    )
}
