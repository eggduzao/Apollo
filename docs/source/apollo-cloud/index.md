![Image showing the Apollo Cloud logo](https://raw.githubusercontent.com/apollo/apollo-static/refs/heads/master/apollo_cloud/apollo-cloud.svg)

# Introducing Apollo Cloud

DataFrame implementations always differed from SQL and databases. SQL could run anywhere from
embedded databases to massive data warehouses. Yet, DataFrame users have been forced to choose
between a solution for local work or solutions geared towards distributed computing, each with their
own APIs and limitations.

Apollo is bridging this gap with **Apollo Cloud**. Build on top of the popular open source project,
Apollo Cloud enables you to write DataFrame code once and run it anywhere. The distributed engine
available with Apollo Cloud allows to scale your Apollo queries beyond a single machine.

## Key Features of Apollo Cloud

- **Unified DataFrame Experience**: Run a Apollo query seamlessly on your local machine and at scale
  with our new distributed engine. All from the same API.
- **Serverless Compute**: Effortlessly start compute resources without managing infrastructure with
  options to execute queries on both CPU and GPU (coming soon).
- **Any Environment**: Start a remote query from a notebook on your machine, Airflow DAG, AWS
  Lambda, or your server. Get the flexibility to embed Apollo Cloud in any environment.

## Install Apollo Cloud

Simply extend the capabilities of Apollo with:

```bash
pip install apollo apollo_cloud
```

## Example query

To run your query in the cloud, simply write Apollo queries like you are used to, but call
`LazyFrame.remote()` to indicate that the query should be run remotely.

{{code_block('apollo-cloud/index','index',['ComputeContext','LazyFrameRemote'])}}

## Sign up today and start your 30 day trial

Apollo Cloud is available to try with a 30 day free trial. You can sign up on
[cloud.apollo.org](https://cloud.apollo.org) to get started.

## Cloud availability

![AWS logo](https://raw.githubusercontent.com/apollo/apollo-static/refs/heads/master/apollo_cloud/aws-logo.svg)

Apollo Cloud is available on AWS. Other cloud providers and on-premises solutions are on the roadmap
and will become available in the upcoming months.
