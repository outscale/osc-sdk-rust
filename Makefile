API_VERSION=$(shell cat api_version)
SDK_VERSION=$(shell cat sdk_version)
USER_ID=$(shell id -u)
GROUP_ID=$(shell id -g)
OPENAPI_IMAGE=outscale/openapi-generator:v5.3.1-rust-awsv4

all: help

.PHONY: help
help:
	@echo "help:"
	@echo "- make gen  : regenerate SDK"
	@echo "- make test : run all tests"

.PNONY: openapi-generator-help
openapi-generator-help:
	docker run --rm openapitools/openapi-generator-cli$(OPENAPI_GEN_VERSION) config-help -g rust

.PHONY: gen
gen: clean osc-api/outscale.yaml
	rm -rf .sdk || true
	mkdir .sdk
	docker run -v $(PWD):/sdk --rm $(OPENAPI_IMAGE) generate -i /sdk/osc-api/outscale.yaml -g rust -c /sdk/gen.yml -o /sdk/.sdk --additional-properties=packageVersion=$(SDK_VERSION)
	# Set default user agent including sdk version using reproductible sed.
	docker run -v $(PWD):/sdk --rm $(OPENAPI_IMAGE) sed -i "s/ *user_agent: Some.*/            user_agent: Some(\"osc\-sdk\-rust\/$(SDK_VERSION)\".to_owned()),/" /sdk/.sdk/src/apis/configuration.rs
	# Set outscale as author
	docker run -v $(PWD):/sdk --rm $(OPENAPI_IMAGE) sed -i "s/OpenAPI Generator team and contributors/Outscale SAS <opensource@outscale.com>/g" /sdk/.sdk/Cargo.toml
	# Set rust version, licensing, homepage, description, ...
	docker run -v $(PWD):/sdk --rm $(OPENAPI_IMAGE) sed -i "s/edition = \"2018\"/edition = \"2021\"\nlicense = \"BSD-3-Clause\"\ndescription = \"Outscale API SDK\"\nhomepage = \"https:\/\/github.com\/outscale\/osc-sdk-rust\/\"/" /sdk/.sdk/Cargo.toml
	# Add dev-dependencies (used by examples)
	docker run -v $(PWD):/sdk --rm $(OPENAPI_IMAGE) sed -i "s/\[dev-dependencies\]/\[dev-dependencies\]\nrand = \"0.8.5\"/" /sdk/.sdk/Cargo.toml
	docker run -v $(PWD):/sdk --rm $(OPENAPI_IMAGE) chown -R $(USER_ID).$(GROUP_ID) /sdk/.sdk
	rm -rf .sdk/git_push.sh
	mv .sdk/README.md .sdk/docs/README.md
	mv .sdk/* .
	rm -rf .sdk
	cargo fmt
	# Apply additional Outscale features or fixes which are not included in generation
	git apply .patches/*
	cargo fmt

osc-api/outscale.yaml:
	git clone https://github.com/outscale/osc-api.git && cd osc-api && git checkout -b $(API_VERSION) $(API_VERSION)

.PHONY: clean
clean:
	rm -rf .sdk osc-api Cargo.toml docs src || true

.PHONY: test
test: build-test reuse-test examples-test regen-test
	@echo all tests OK

.PHONY: build-test
build-test:
	cargo build --all-targets

.PHONY: reuse-test
reuse-test:
	docker run --rm --volume $(PWD):/data fsfe/reuse:0.11.1 lint

.PHONY: examples-test
examples-test:
	cargo run --example volume
	cargo run --example region
	cargo run --example keypair
	cargo run --example config_file

# try to regen, should not have any difference
.PHONY: regen-test
regen-test: gen
	git add Cargo.toml docs src
	git diff --cached -s --exit-code
	git diff -s --exit-code

# Used by bot to auto-release
# GH_TOKEN and SSH_PRIVATE_KEY are needed
.PHONY: auto-release
auto-release: auto-release-cleanup osc-api-check release-check-duplicate release-build release-push release-pr
	@echo OK

.PHONY: auto-release-cleanup
auto-release-cleanup:
	rm -rf .auto-release-abort || true

.PHONY: osc-api-check
osc-api-check:
	bash .github/scripts/osc-api-check.sh

.PHONY: release-check-duplicate
release-check-duplicate:
	bash .github/scripts/release-check-duplicate.sh

.PHONY: release-build
release-build:
	bash .github/scripts/release-build.sh

.PHONY: release-push
release-push:
	bash .github/scripts/release-push.sh

.PHONY: release-pr
release-pr:
	bash .github/scripts/release-pr.sh
