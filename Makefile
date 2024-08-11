.DEFAULT_GOAL := help

.PHONY: build help

build: ## Build containers with current user's UID and GID
	@echo "Building containers with UID=$(shell id -u) and GID=$(shell id -g)"
	UID=$(shell id -u) GID=$(shell id -g) docker compose build

migrate: ## Migrate database
	@docker compose run --rm backend diesel migration run

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sed -n 's/^\(.*\): \(.*\)##\(.*\)/• \1: \3/p'
