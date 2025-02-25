PYTHON_EXT_PACKAGE_NAME="octatools-py"
PYTHON_EXT_MODULE_NAME=octatools_py

all:
	docs test build checks

examples-yaml-dumps:
	rm -f ./examples/yaml/dumps/*
	cargo r banks dump data/tests/blank-project/bank01.work ./examples/yaml/dumps/bank.yaml >/dev/null 2>&1
	cargo r projects dump data/tests/blank-project/project.work ./examples/yaml/dumps/project.yaml >/dev/null 2>&1
	cargo r drive dump data/tests/drive/DEMO-DRIVE-DATA/ ./examples/yaml/dumps/drive.yaml >/dev/null 2>&1

examples-json-dumps:
	echo "TODO"
	# rm -f ./examples/json/dumps/*
	# cargo r banks dump json data/tests/blank-project/bank01.work ./examples/yaml/dumps/bank.yaml
	# cargo r projects dump json data/tests/blank-project/project.work ./examples/yaml/dumps/project.yaml

install-qlty:
	curl https://qlty.sh | bash
	qlty init


checks:
	qlty check --filter clippy
	qlty smells ./serde_octatrack/ ./octatools-*/

lint:
	cargo fmt --all

fixup: lint
	qlty check --filter clippy --fix

docs-full:
	cargo doc --workspace

docs:
	cargo doc --workspace --no-deps

test:
	cargo test --workspace --no-fail-fast

cov:
	cargo tarpaulin --workspace

build:
	cargo build --workspace

setup-py:
	virtualenv -p python3 ./${PYTHON_EXT_PACKAGE_NAME}/venv/
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -m pip install maturin

build-py: setup-py
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/maturin build --manifest-path ./${PYTHON_EXT_PACKAGE_NAME}/Cargo.toml
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -m pip install --force-reinstall $(wildcard ./target/wheels/octatools_py-*.whl)

smoke-py: build-py
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.bank_file_to_json(\"./data/tests/blank-project/bank01.work\")).keys(); print('arrangment:', keys)"
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.arrangement_file_to_json(\"./data/tests/blank-project/arr01.work\")).keys(); print('bank:', keys)"
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.project_file_to_json(\"./data/tests/blank-project/project.work\")).keys(); print('project:', keys)"
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.sample_attributes_file_to_json(\"./data/tests/misc/pair.ot\")).keys(); print('sample attibutes:', keys)"

run:
	cargo run

release:
	cargo build --workspace --release

clean:
	rm -rf ./target/

