ENDPOINT ?= polygon.substreams.pinax.network:443
START_BLOCK ?= -1000
STOP_BLOCK ?= 0
PARALLEL_JOBS ?= 500
.DEFAULT_GOAL := pack

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack -o spkg/{spkgDefaultName}

.PHONY: noop
noop: build
	substreams-sink-noop $(ENDPOINT) substreams.yaml db_out -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)" $(START_BLOCK):$(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) --limit-processed-blocks 0

.PHONY: prod
prod: build
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK) --limit-processed-blocks 0 --production-mode -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)"
