use sc_transaction_pool::BasicPool;
use sc_transaction_pool::ChainApi;
use sc_transaction_pool_api::{
	ImportNotificationStream, PoolFuture, PoolStatus, ReadyTransactions, TransactionFor,
	TransactionPool, TransactionSource, TransactionStatusStreamFor, TxHash,
};
use sp_runtime::traits::{Block as BlockT, NumberFor};
use futures::Future;
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub struct CustomPool<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: ChainApi<Block = Block>,
{
	inner_pool: BasicPool<PoolApi, Block>,
}

impl<PoolApi: ChainApi<Block = Block> + 'static, Block: BlockT> TransactionPool
	for CustomPool<PoolApi, Block>
{
	type Block = <BasicPool<PoolApi, Block> as TransactionPool>::Block;
	type Hash = <BasicPool<PoolApi, Block> as TransactionPool>::Hash;
	type InPoolTransaction = <BasicPool<PoolApi, Block> as TransactionPool>::InPoolTransaction;
	type Error = <BasicPool<PoolApi, Block> as TransactionPool>::Error;

	fn submit_at(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xts: Vec<TransactionFor<Self>>,
	) -> PoolFuture<Vec<Result<TxHash<Self>, Self::Error>>, Self::Error> {
		self.inner_pool.submit_at(at, source, xts)
	}

	fn submit_one(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> PoolFuture<TxHash<Self>, Self::Error> {
		self.inner_pool.submit_one(at, source, xt)
	}

	fn submit_and_watch(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> PoolFuture<Pin<Box<TransactionStatusStreamFor<Self>>>, Self::Error> {
        self.inner_pool.submit_and_watch(at, source, xt)
	}

	fn remove_invalid(&self, hashes: &[TxHash<Self>]) -> Vec<Arc<Self::InPoolTransaction>> {
		self.inner_pool.remove_invalid(hashes)
	}

	fn status(&self) -> PoolStatus {
		self.inner_pool.status()
	}

	fn import_notification_stream(&self) -> ImportNotificationStream<TxHash<Self>> {
		self.inner_pool.import_notification_stream()
	}

	fn hash_of(&self, xt: &TransactionFor<Self>) -> TxHash<Self> {
		self.inner_pool.hash_of(xt)
	}

	fn on_broadcasted(&self, propagations: HashMap<TxHash<Self>, Vec<String>>) {
		self.inner_pool.on_broadcasted(propagations)
	}

	fn ready_transaction(&self, hash: &TxHash<Self>) -> Option<Arc<Self::InPoolTransaction>> {
		self.inner_pool.ready_transaction(hash)
	}

	fn ready_at(
		&self,
		at: NumberFor<Self::Block>,
	) -> Pin<
		Box<
			dyn Future<
					Output = Box<dyn ReadyTransactions<Item = Arc<Self::InPoolTransaction>> + Send>,
				> + Send,
		>,
	> {
		self.inner_pool.ready_at(at)
	}

	fn ready(&self) -> Box<dyn ReadyTransactions<Item = Arc<Self::InPoolTransaction>> + Send> {
		self.inner_pool.ready()
	}

	fn futures(&self) -> Vec<Self::InPoolTransaction> {
		self.inner_pool.futures()
	}
}
