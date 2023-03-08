.PHONY: default

default:
	cargo snippet -t vscode > rust-comp-snippets.json

snippet:
	cargo snippet -t vscode > ../beginners/.vscode/cargo_snippet.code-snippets
