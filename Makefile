.PHONY = parser
puddlejumper: parser
	cd puddlejumper && cargo build --release

.PHONY = test
test:
	cd tree-sitter-puddlejumper && npm run test

.PHONY = parser
parser:
	cd tree-sitter-puddlejumper && npm run build && cargo build --release
