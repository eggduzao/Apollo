use std::hash::Hash;

use apollo_compute::hyperloglogplus::HyperLogLog;
use apollo_utils::IdxSize;
use apollo_utils::total_ord::{ToTotalOrd, TotalEq, TotalHash};

use super::{ChunkApproxNUnique, ChunkedArray, ApolloDataType};

impl<T> ChunkApproxNUnique for ChunkedArray<T>
where
    T: ApolloDataType,
    for<'a> T::Physical<'a>: TotalHash + TotalEq + Copy + ToTotalOrd,
    for<'a> <Option<T::Physical<'a>> as ToTotalOrd>::TotalOrdItem: Hash + Eq,
{
    fn approx_n_unique(&self) -> IdxSize {
        let mut hllp = HyperLogLog::new();
        self.iter().for_each(|item| hllp.add(&item.to_total_ord()));
        hllp.count() as IdxSize
    }
}
