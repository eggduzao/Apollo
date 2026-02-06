# Set up organization

Organizations in Apollo Cloud are the top-level entity and typically represent a company. They can
have members, contain multiple workspaces and are used to manage billing.

To set up an organization you can either use [the dashboard](https://cloud.apollo.org/portal/) or the
following CLI commands:

- If you're just starting with Apollo Cloud then you need to set up both an organization and
  workspace:

  ```bash
  pc setup
  ```

- Or if you only want to set up an organization:

  ```bash
  pc organization setup
  ```

For other subcommands you can execute the help command:

```bash
pc organization --help 

usage: pc organization [-h] [-v] [-t TOKEN] [-p TOKEN_PATH] {list,setup,delete,details} ...

positional arguments:
  {list,setup,delete,details}
    list                List all active organizations
    setup               Set up an organization
    delete              Delete an organization
    details             Print the details of an organization
```
