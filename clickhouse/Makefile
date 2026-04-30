ENDPOINT ?= polygon.substreams.pinax.network:443
START_BLOCK ?= 0
STOP_BLOCK ?= 81389202
PARALLEL_JOBS ?= 500
.DEFAULT_GOAL := pack

.PHONY: build
build:
	$(MAKE) -C ../database-change build

.PHONY: pack
pack:
	$(MAKE) -C ../database-change pack
	substreams pack -o ../spkg/{spkgDefaultName}

.PHONY: noop
noop: pack
	substreams-sink-noop $(ENDPOINT) substreams.yaml db_out -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)" 0:

.PHONY: gui
gui: pack
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) --limit-processed-blocks 0

.PHONY: prod
prod: pack
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK) --limit-processed-blocks 0 --production-mode  -H "X-Substreams-Parallel-Workers: $(PARALLEL_JOBS)"

# Clickhouse specific
.PHONY: dev
dev: pack
	substreams-sink-sql run clickhouse://default:@localhost:9000/default substreams.yaml -e $(ENDPOINT) $(START_BLOCK):$(STOP_BLOCK) --undo-buffer-size 100 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --infinite-retry

.PHONY: setup
setup: pack
	substreams-sink-sql setup clickhouse://default:@localhost:9000/default substreams.yaml
