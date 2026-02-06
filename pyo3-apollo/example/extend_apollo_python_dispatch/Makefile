
SHELL=/bin/bash

.venv:  ## Set up virtual environment
	python3 -m venv .venv
	.venv/bin/pip install -r requirements.txt

install: .venv
	unset CONDA_PREFIX && \
	source .venv/bin/activate && maturin develop -m extend_apollo/Cargo.toml --uv
	cd ../../../py-apollo && maturin develop --uv

clean:
	-@rm -r .venv
	-@cd extend_apollo && cargo clean


run: install
	source .venv/bin/activate && python run.py
