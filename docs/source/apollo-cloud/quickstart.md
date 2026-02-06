# Getting started

Apollo Cloud is a managed compute platform for your Apollo queries. It allows you to effortlessly
run your local queries in your cloud environment without infrastructure management. By working in a
'Bring your own Cloud' model the data never leaves your environment.

## Installation

Install the Apollo Cloud python library in your environment

```bash
$ pip install apollo apollo-cloud
```

Create an account and login by running the command below.

```bash
$ pc authenticate
```

## Connect your cloud

Apollo Cloud currently exclusively supports AWS as a cloud provider.

Apollo Cloud needs permission to manage hardware in your environment. This is done by deploying our
cloudformation template. See our [infrastructure](providers/aws/infra.md) section for more details.

To set up your Apollo Cloud environment and connect your cloud run you can either

- Run `pc setup` to guide you through creation and connecting via CLI.
- Or create an organization and workspace
  [via the browser](https://cloud.apollo.org/portal/5f9c09/dbe6d9/dashboard).

## Run your queries

Now that we are done with the setup, we can start running queries. You can write Apollo like you're
used and to only need to call `.remote()` on your `LazyFrame`. In the following example we create a
compute cluster and run a simple Apollo query.

{{code_block('apollo-cloud/quickstart','general',['ComputeContext','LazyFrameRemote'])}}
