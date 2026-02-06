# CLI

Apollo cloud comes with a command line interface (CLI) out of the box. This allows you to interact
with apollo cloud resources from the terminal.

```bash
pc --help

usage: pc [-h] [-v] [-V] {authenticate,setup,login,organization,workspace,compute} ...

positional arguments:
  {authenticate,setup,login,organization,workspace,compute}
    authenticate        Authenticate with Apollo Cloud by loading stored credentials or otherwise logging in through the browser.
    setup               Set up an organization and workspace to quickly run queries. Ideal to get started with Apollo Cloud.
    login               Authenticate with Apollo Cloud by logging in through the browser.
    organization        Manage Apollo Cloud organizations.
    workspace           Manage Apollo Cloud workspaces.
    compute             Manage Apollo Cloud compute clusters.

optional arguments:
  -h, --help            show this help message and exit.
  -v, --verbose         Output debug logging messages.
  -V, --version         Display the version of the Apollo Cloud client.
```

If you're just starting out with Apollo Cloud then `pc setup` will guide you through setting up your
environment to be able to quickly run queries.
