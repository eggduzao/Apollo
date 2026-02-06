# Introducing Apollo on-premises

Interested in running Apollo on-premises?
[Sign up here to apply](https://w0lzyfh2w8o.typeform.com/to/zuoDgoMv).

After installing Apollo on-premises either on bare-metal or on Kubernetes, you can connect to your
cluster using the Apollo Cloud Python client.

```python
import apollo as pl
import apollo_cloud as pc

# Connect to your Apollo on-premises cluster
ctx = pc.ClusterContext(compute_address="your-cluster-compute-address", insecure=True)
query = (
    pl.LazyFrame()
    .with_columns(a=pl.arange(0, 100000000).sum())
    .remote(ctx)
    .distributed()
    .execute()
)
print(query.await_result())
```
