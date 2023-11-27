.DEFAULT: help
.PHONY: help
help:
	@grep -E -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: today
today: ## Run today's program
today:
	cargo run --bin day$(shell date +%d)
