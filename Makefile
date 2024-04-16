X86_64_TAG = "x86_64-unknown-linux-gnu"
BUILD_PATH_X86_64 = "target/$(X86_64_TAG)/release"
AARCH64_TAG = "aarch64-unknown-linux-gnu"
BUILD_PATH_AARCH64 = "target/$(AARCH64_TAG)/release"

ifeq ($(OS),Windows_NT)
    FEATURES?=
else
    FEATURES?=jemalloc
endif

CROSS_FEATURES ?= jemalloc

CROSS_PROFILE ?= release

PROFILE ?= release

install:
	cargo install --path contower --force --locked \
		--features "$(FEATURES)" \
		--profile "$(PROFILE)"