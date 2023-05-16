.PHONY = parser
puddlejumper: parser
	cd puddlejumper && cargo build --release

.PHONY = parser
parser:
	cd tree-sitter-puddlejumper && npm run build && cargo build --release