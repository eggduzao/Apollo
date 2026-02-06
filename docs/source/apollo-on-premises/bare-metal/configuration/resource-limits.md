# Resource limits

Apollo will always attempt to use all available CPU resources, and consumes memory resources as
needed. If possible, assign physical cores to Apollo to avoid contention with other processes.
Apollo on-premises workers consist of a main process and an executor process, with the latter doing
most computation. If the executor dies, it will lose all progress of the stage it was working on.
All progress of previous stages (i.e. shuffle data) is managed by the main process.

In other words, if the system is low on memory, the first process that should be killed, is the
executor process. Apollo on-premises will already automatically configure `oom-score-adj` on its
executor process.

If there are other system critical processes, we recommend either delegating a cgroup to Apollo
on-premises, or manually setting up cgroup limits for the entire Apollo on-premises service.

### Delegating cgroup to Apollo on-premises

Cgroups can contain subgroups, each with independent limits. Apollo on-premises can create these
cgroups, and choose proper memory limits for each of its components. To use this feature, ensure you
delegate cgroups to the Apollo On-Premise process and configure `memory_limit` in the configuration
file.

```toml
cluster_id = "apollo-cluster"
instance_id = "node-0"
license = "/etc/apollo/license.json"
memory_limit = 10737418240 # 10 GiB
# ...
```

### Manually configuring cgroup limits

You can also manually configure a memory limit on a cgroup containing all the processes. For example
using Systemd's `resource-control`. The disadvantage of this approach is that the individual
components will contend for the same memory capacity, which may prevent Apollo on-premises from
gracefully handling OOM errors on the executor.
