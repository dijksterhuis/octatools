PYTHON_EXT_PACKAGE_NAME="ot-tools-py"
PYTHON_EXT_MODULE_NAME="ot_tools_py"

all:
	docs test build checks

gen-examples-human-readable:
	echo "Generating YAML example files ..."
	rm -f ./examples/human-readable/yaml
	mkdir -p ./examples/human-readable/yaml
	cargo r bin-files bin-to-human bank data/tests/blank-project/bank01.work yaml ./examples/human-readable/yaml/bank01.yaml >/dev/null 2>&1
	cargo r bin-files bin-to-human bank data/tests/blank-project/project.work yaml ./examples/human-readable/yaml/project.yaml >/dev/null 2>&1
	cargo r bin-files bin-to-human bank data/tests/blank-project/arr01.work yaml ./examples/human-readable/yaml/arr01.yaml >/dev/null 2>&1
	echo "Generating JSON example files ..."
	rm -f ./examples/human-readable/json
	mkdir -p ./examples/human-readable/json
	cargo r bin-files bin-to-human bank data/tests/blank-project/bank01.work json ./examples/human-readable/json/bank01.json >/dev/null 2>&1
	cargo r bin-files bin-to-human bank data/tests/blank-project/project.work json ./examples/human-readable/json/project.json >/dev/null 2>&1
	cargo r bin-files bin-to-human bank data/tests/blank-project/arr01.work json ./examples/human-readable/json/arr01.json >/dev/null 2>&1
	echo "Done."

install-qlty:
	curl https://qlty.sh | bash
	qlty init


checks:
	qlty check --filter clippy
	qlty smells ./ot-tools-*/

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
	cargo tarpaulin --workspace --exclude ot-tools-py --exclude ot-tools-derive

build:
	cargo build --workspace

setup-py:
	virtualenv -p python3 ./${PYTHON_EXT_PACKAGE_NAME}/venv/
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -m pip install maturin

build-py: setup-py
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/maturin build --manifest-path ./${PYTHON_EXT_PACKAGE_NAME}/Cargo.toml
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -m pip install --force-reinstall $(wildcard ./target/wheels/ot_tools_py*.whl)

smoke-py: build-py
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.bank_file_to_json(\"./data/tests/blank-project/bank01.work\")).keys(); print('arrange:', keys)"
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.arrangement_file_to_json(\"./data/tests/blank-project/arr01.work\")).keys(); print('bank:', keys)"
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.project_file_to_json(\"./data/tests/blank-project/project.work\")).keys(); print('project:', keys)"
	./${PYTHON_EXT_PACKAGE_NAME}/venv/bin/python3 -Bc "import ${PYTHON_EXT_MODULE_NAME}, json; keys = json.loads(${PYTHON_EXT_MODULE_NAME}.sample_attributes_file_to_json(\"./data/tests/misc/pair.ot\")).keys(); print('sample:', keys)"

run:
	cargo run

release:
	cargo build --workspace --release

clean:
	rm -rf ./target/

