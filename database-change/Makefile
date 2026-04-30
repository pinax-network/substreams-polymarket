ENDPOINT ?= polygon.substreams.pinax.network:443
START_BLOCK ?= 80223813
STOP_BLOCK ?= 0
PARALLEL_JOBS ?= 500
.DEFAULT_GOAL := pack

.PHONY: build
build:
	cargo build -p database-change --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	$(MAKE) -C .. pack
	substreams pack -o ../spkg/{spkgDefaultName}

.PHONY: noop
noop: pack
	substreams-sink-noop $(ENDPOINT) substreams.yaml db_out -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)" 4027499:

.PHONY: gui
gui: pack
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) --limit-processed-blocks 0

.PHONY: prod
prod: pack
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK) --limit-processed-blocks 0 --production-mode -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)"
