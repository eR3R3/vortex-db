use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct QueryExecutor(Arc<InnerQueryExecutor>);

pub(super) struct InnerQueryExecutor {
    table: String,
    ft_map: HashMap<IndexReference, FtIndex>,
    mr_entries: HashMap<MatchRef, FtEntry>,
    exp_entries: HashMap<Arc<Expression>, FtEntry>,
    it_entries: Vec<IteratorEntry>,
    mt_entries: HashMap<Arc<Expression>, MtEntry>,
    hnsw_entries: HashMap<Arc<Expression>, HnswEntry>,
    knn_bruteforce_entries: HashMap<Arc<Expression>, KnnBruteForceEntry>,
}

