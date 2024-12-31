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

run:
	cargo run

release:
	cargo build --workspace --release

clean:
	rm -rf ./target/

