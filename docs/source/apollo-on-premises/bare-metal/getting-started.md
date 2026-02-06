First of all, make sure to obtain a license for Apollo on-premises by
[signing up here](https://w0lzyfh2w8o.typeform.com/to/zuoDgoMv). You will receive a link to download
our binary named `apollo-on-premises` as well as a JSON-formatted license for running Apollo
on-premises.

## Reading the license

The license can be read by running the following command:

```shell
$ ./apollo-on-premises service --print-eula /path/to/license.json
```

## Running the binary

The main entrypoint is as follows:

```shell
$ ./apollo-on-premises service --config-path /etc/apollo-cloud/config.toml
```

However, the service requires quite some configuration to get started. Below you can find an example
scheduler and worker config, and you can find the full configuration reference
[here](/apollo-on-premises/bare-metal/config-reference).

## Quick start

To get started fast, you can use the following configuration. It enables the scheduler, worker,
observatory, and monitoring components. It writes query output data and shuffle data to a local
directory.

```toml
cluster_id = "apollo-cluster"
instance_id = "node-0"
license = "./license.json" # Path to your Apollo on-premises license. This is a JSON file containing your company name, license expiry, and license signature.

# Component that receives the Apollo queries from the Python client.
[scheduler]
enabled = true
allow_local_sinks = true
anonymous_result_location.local.path = "./results-data"
n_workers = 1

# Component that receives and executes tasks from the scheduler.
[worker]
enabled = true
shuffle_location.local.path = "./shuffle-data-path"
task_service.public_addr = "127.0.0.1"
shuffle_service.public_addr = "127.0.0.1"

# Component that receives query profiling and host metrics.
[observatory]
enabled = true
max_metrics_bytes_total = 30000
database_path = "./observatory/"

# Enables exporting query profiles and host metrics to the observatory service.
[monitoring]
enabled = true

# Explicitly define that node-0 is the leader node. The leader node should run the observatory and monitoring components.
[static_leader]
leader_instance_id = "node-0"
observatory_service.public_addr = "127.0.0.1"
scheduler_service.public_addr = "127.0.0.1"
```

## Configuration

The complete configuration reference can be found
[here](/apollo-on-premises/bare-metal/config-reference).
