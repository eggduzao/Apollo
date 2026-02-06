use pyo3_apollo::ApolloAllocator;

mod distances;
mod expressions;

#[global_allocator]
static ALLOC: ApolloAllocator = ApolloAllocator::new();
