//! Helper types to workaround 'higher-ranked lifetime error'
//! <https://github.com/rust-lang/rust/issues/100013> in default implementation of
//! `reth_rpc_eth_api::helpers::Call`.

use reth_primitives::{B256, U256};
use reth_provider::StateProvider;
use reth_revm::{database::StateProviderDatabase, db::CacheDB, DatabaseRef};
use revm::Database;

/// Helper alias type for the state's [`CacheDB`]
pub type StateCacheDb<'a> = CacheDB<StateProviderDatabase<StateProviderTraitObjWrapper<'a>>>;

/// Hack to get around 'higher-ranked lifetime error', see
/// <https://github.com/rust-lang/rust/issues/100013>
#[allow(missing_debug_implementations)]
pub struct StateProviderTraitObjWrapper<'a>(pub &'a dyn StateProvider);

impl<'a> reth_provider::StateRootProvider for StateProviderTraitObjWrapper<'a> {
    fn state_root(
        &self,
        bundle_state: &revm::db::BundleState,
    ) -> reth_errors::ProviderResult<B256> {
        self.0.state_root(bundle_state)
    }

    fn state_root_with_updates(
        &self,
        bundle_state: &revm::db::BundleState,
    ) -> reth_errors::ProviderResult<(B256, reth_trie::updates::TrieUpdates)> {
        self.0.state_root_with_updates(bundle_state)
    }
}

impl<'a> reth_provider::StateProofProvider for StateProviderTraitObjWrapper<'a> {
    fn proof(
        &self,
        state: &revm::db::BundleState,
        address: revm_primitives::Address,
        slots: &[B256],
    ) -> reth_errors::ProviderResult<reth_trie::AccountProof> {
        self.0.proof(state, address, slots)
    }
}

impl<'a> reth_provider::AccountReader for StateProviderTraitObjWrapper<'a> {
    fn basic_account(
        &self,
        address: revm_primitives::Address,
    ) -> reth_errors::ProviderResult<Option<reth_primitives::Account>> {
        self.0.basic_account(address)
    }
}

impl<'a> reth_provider::BlockHashReader for StateProviderTraitObjWrapper<'a> {
    fn block_hash(
        &self,
        block_number: reth_primitives::BlockNumber,
    ) -> reth_errors::ProviderResult<Option<B256>> {
        self.0.block_hash(block_number)
    }

    fn canonical_hashes_range(
        &self,
        start: reth_primitives::BlockNumber,
        end: reth_primitives::BlockNumber,
    ) -> reth_errors::ProviderResult<Vec<B256>> {
        self.0.canonical_hashes_range(start, end)
    }

    fn convert_block_hash(
        &self,
        hash_or_number: reth_rpc_types::BlockHashOrNumber,
    ) -> reth_errors::ProviderResult<Option<B256>> {
        self.0.convert_block_hash(hash_or_number)
    }
}

impl<'a> StateProvider for StateProviderTraitObjWrapper<'a> {
    fn account_balance(
        &self,
        addr: revm_primitives::Address,
    ) -> reth_errors::ProviderResult<Option<U256>> {
        self.0.account_balance(addr)
    }

    fn account_code(
        &self,
        addr: revm_primitives::Address,
    ) -> reth_errors::ProviderResult<Option<reth_primitives::Bytecode>> {
        self.0.account_code(addr)
    }

    fn account_nonce(
        &self,
        addr: revm_primitives::Address,
    ) -> reth_errors::ProviderResult<Option<u64>> {
        self.0.account_nonce(addr)
    }

    fn bytecode_by_hash(
        &self,
        code_hash: B256,
    ) -> reth_errors::ProviderResult<Option<reth_primitives::Bytecode>> {
        self.0.bytecode_by_hash(code_hash)
    }

    fn storage(
        &self,
        account: revm_primitives::Address,
        storage_key: reth_primitives::StorageKey,
    ) -> reth_errors::ProviderResult<Option<reth_primitives::StorageValue>> {
        self.0.storage(account, storage_key)
    }
}

/// Hack to get around 'higher-ranked lifetime error', see
/// <https://github.com/rust-lang/rust/issues/100013>
#[allow(missing_debug_implementations)]
pub struct StateCacheDbRefMutWrapper<'a, 'b>(pub &'b mut StateCacheDb<'a>);

impl<'a, 'b> Database for StateCacheDbRefMutWrapper<'a, 'b> {
    type Error = <StateCacheDb<'a> as Database>::Error;
    fn basic(
        &mut self,
        address: revm_primitives::Address,
    ) -> Result<Option<revm_primitives::AccountInfo>, Self::Error> {
        self.0.basic(address)
    }

    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        self.0.block_hash(number)
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<revm_primitives::Bytecode, Self::Error> {
        self.0.code_by_hash(code_hash)
    }

    fn storage(
        &mut self,
        address: revm_primitives::Address,
        index: U256,
    ) -> Result<U256, Self::Error> {
        self.0.storage(address, index)
    }
}

impl<'a, 'b> DatabaseRef for StateCacheDbRefMutWrapper<'a, 'b> {
    type Error = <StateCacheDb<'a> as Database>::Error;

    fn basic_ref(
        &self,
        address: revm_primitives::Address,
    ) -> Result<Option<revm_primitives::AccountInfo>, Self::Error> {
        self.0.basic_ref(address)
    }

    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        self.0.block_hash_ref(number)
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<revm_primitives::Bytecode, Self::Error> {
        self.0.code_by_hash_ref(code_hash)
    }

    fn storage_ref(
        &self,
        address: revm_primitives::Address,
        index: U256,
    ) -> Result<U256, Self::Error> {
        self.0.storage_ref(address, index)
    }
}
