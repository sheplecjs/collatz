.DEFAULT_GOAL := help

RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[1;33m
BLUE = \033[0;34m
NO_COLOR = \033[0m

PROJECT_NAME ?= Collatz

## Format, compile, and run
all: format compile run

## Compile to binary
compile:
	@cargo build


## Run clang-format
format:
	@cargo fmt

## Run the executable
run:
	@./target/debug/collatz

## Create volumes for compose environments
volumes:
	@docker volume create pg-data
	@docker volume create collatz_notebooks

## Run compose environment
compose:
	@docker-compose -f docker-compose-collatz.yml run --rm collatz

## Run analysis environment
analysis:
	@docker-compose -f docker-compose-analysis.yml up

## This help screen
help:
	@printf "Available targets:\n\n"
	@awk '/^[a-zA-Z\-\_0-9%:\\]+/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
		helpCommand = $$1; \
		helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
	gsub("\\\\", "", helpCommand); \
	gsub(":+$$", "", helpCommand); \
		printf "  \x1b[32;01m%-35s\x1b[0m %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST) | sort -u
	@printf "\n"