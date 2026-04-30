ENDPOINT ?= polygon.substreams.pinax.network:443
START_BLOCK ?= 0
STOP_BLOCK ?= 0
PARALLEL_JOBS ?= 5000
.DEFAULT_GOAL := pack

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: build
build:
	cargo build -p polymarket --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack -o spkg/{spkgDefaultName}

.PHONY: noop
noop: build
	substreams-sink-noop $(ENDPOINT) substreams.yaml map_events -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)" 4027499:

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) --limit-processed-blocks 0

.PHONY: prod
prod: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) -t $(STOP_BLOCK) --limit-processed-blocks 0 --production-mode -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)"
