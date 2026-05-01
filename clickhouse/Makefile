ENDPOINT ?= polygon.substreams.pinax.network:443
START_BLOCK ?= 86220000
STOP_BLOCK ?= 86221000
PARALLEL_JOBS ?= 500
.DEFAULT_GOAL := pack

.PHONY: schema
schema:
	@echo "-- This file is generated. Do not edit." > schema.sql
	@echo "" >> schema.sql
	@for f in $$(ls schema.*.sql 2>/dev/null | sort); do \
		cat "$$f" >> schema.sql; \
		echo "" >> schema.sql; \
	done

.PHONY: pack
pack: schema
	substreams pack

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
	substreams-sink-sql run clickhouse://default:@localhost:9000/default substreams.yaml -e $(ENDPOINT) $(START_BLOCK): --undo-buffer-size 0 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --live-block-flush-interval 1 --infinite-retry --development-mode

.PHONY: setup
setup: pack
	substreams-sink-sql setup clickhouse://default:@localhost:9000/default substreams.yaml
