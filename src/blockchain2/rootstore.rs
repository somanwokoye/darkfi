use sled::Batch;

use crate::{
    crypto::merkle_node::MerkleNode,
    util::serial::{deserialize, serialize},
    Result,
};

const SLED_ROOTS_TREE: &[u8] = b"_merkleroots";

pub struct RootStore(sled::Tree);

impl RootStore {
    /// Opens a new or existing `RootStore` on the given sled database.
    pub fn new(db: &sled::Db) -> Result<Self> {
        let tree = db.open_tree(SLED_ROOTS_TREE)?;
        Ok(Self(tree))
    }

    /// Insert a slice of [`MerkleNode`] on the given sled database.
    pub fn insert(&self, roots: &[MerkleNode]) -> Result<()> {
        let mut batch = Batch::default();
        for i in roots {
            batch.insert(serialize(i), vec![] as Vec<u8>);
        }

        self.0.apply_batch(batch)?;
        Ok(())
    }

    /// Retrieve all merkle roots.
    /// Be careful as this will try to load everything in memory.
    pub fn get_all(&self) -> Result<Vec<Option<MerkleNode>>> {
        let mut roots = vec![];
        let mut iterator = self.0.into_iter().enumerate();
        while let Some((_, r)) = iterator.next() {
            let (k, _) = r.unwrap();
            let root = deserialize(&k)?;
            roots.push(Some(root));
        }

        Ok(roots)
    }
}
