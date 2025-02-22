use raft::{
    prelude::*,
    Config, RawNode, StateRole,
};
use slog::Logger;
use std::sync::Arc;
use crate::store::Store;
use super::state::RaftStorage;

pub struct RaftNode {
    id: u64,
    node: RawNode<RaftStorage>,
    store: Arc<Store>,
    logger: Logger,
}

impl RaftNode {
    pub fn new(
        id: u64,
        peers: Vec<u64>,
        store: Arc<Store>,
        logger: Logger,
    ) -> raft::Result<Self> {
        let config = Config {
            id,
            election_tick: 10,
            heartbeat_tick: 3,
            ..Default::default()
        };

        let storage = RaftStorage::new(store.clone(), logger.clone());
        let node = RawNode::new(&config, storage, &logger)?;

        Ok(Self { 
            id,
            node,
            store,
            logger,
        })
    }

    pub async fn tick(&mut self) {
        self.node.tick();
    }

    pub async fn propose(&mut self, data: Vec<u8>) -> raft::Result<()> {
        self.node.propose(vec![], data)?;
        Ok(())
    }

    pub async fn step(&mut self, msg: Message) -> raft::Result<()> {
        self.node.step(msg)?;
        Ok(())
    }

    pub async fn advance(&mut self) {
        self.node.advance(Ready::default());
    }

    pub async fn campaign(&mut self) -> raft::Result<()> {
        self.node.campaign()?;
        Ok(())
    }

    pub fn is_leader(&self) -> bool {
        self.node.raft.state == StateRole::Leader
    }
}