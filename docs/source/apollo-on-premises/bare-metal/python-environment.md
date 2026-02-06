A major point of Apollo on-premises is the system requirements regarding the Python version and
Python dependencies, which need to be identical on the client, the scheduler, and all workers. The
easiest method to achieve this is having a system-wide Python environment and globally installed
packages. We recommend however setting up a virtual environment ([`uv`](https://docs.astral.sh/uv/)
makes this very easy, including maintaining a given Python version).

The minimal requirement for running `apollo-on-premises` is the `apollo` package.

!!! info "Version pinning" Each release of `apollo-on-premises` is pinned to a single `apollo`
release, which can be found in the release announcement.
`shell export PINNED_VERSION=1.35.2 # for instance`

## System-wide installation

```shell
$ uv pip install --break-system-packages -r requirements.txt apollo[cloudpickle]==$PINNED_VERSION
$ ./apollo-on-premises service --config-path /etc/apollo-cloud/config.toml
```

## Virtual Environment

```shell
$ uv venv .venv
$ source .venv/bin/activate
$ export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(uv run python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
$ uv pip install -r requirements.txt apollo[cloudpickle]==$PINNED_VERSION
$ ./apollo-on-premises service --config-path /etc/apollo-cloud/config.toml
```
