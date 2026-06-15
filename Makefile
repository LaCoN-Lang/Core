CARGO := cargo
CROSS := cross

WIN_TARGET := x86_64-pc-windows-msvc
LINUX_TARGET := x86_64-unknown-linux-gnu

RELEASE := --release

# ─────────────────────────────────────────────
# Engine
# ─────────────────────────────────────────────
engine:
	$(CARGO) build $(RELEASE) --package engine

# ─────────────────────────────────────────────
# CLI
# ─────────────────────────────────────────────
cli-win:
	$(CARGO) build $(RELEASE) --package cli --target $(WIN_TARGET)

cli-linux:
	$(CROSS) build $(RELEASE) --package cli --target $(LINUX_TARGET)

cli: cli-win cli-linux

# ─────────────────────────────────────────────
# LSP
# ─────────────────────────────────────────────
lsp-win:
	$(CARGO) build $(RELEASE) --package lsp --target $(WIN_TARGET)

lsp-linux:
	$(CROSS) build $(RELEASE) --package lsp --target $(LINUX_TARGET)

lsp: lsp-win lsp-linux

# ─────────────────────────────────────────────
# WASM
# ─────────────────────────────────────────────
wasm:
	$(CARGO) build $(RELEASE) --package wasm --target wasm32-unknown-unknown

# ─────────────────────────────────────────────
# All
# ─────────────────────────────────────────────
all: engine cli wasm lsp

.PHONY: engine cli-win cli-linux cli lsp-win lsp-linux lsp wasm all
