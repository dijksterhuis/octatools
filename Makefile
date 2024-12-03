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

checks:
	qlty check --filter clippy
	qlty smells ./src/ ./serde_octatrack/

lint:
	cargo fmt

fixup: lint
	qlty check --filter clippy --fix

docs-full:
	cargo doc

docs:
	cargo doc --no-deps

test:
	cargo test

cov:
	cargo tarpaulin

build:
	cargo build

release:
	cargo build --release

clean:
	rm -rf ./target/

