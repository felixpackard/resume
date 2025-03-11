CLI_DIR=packages/cli
WEB_DIR=packages/web
RESUME_JSON=resume.json
SCHEMA_JSON=schema.json

define POST_BUILD_MESSAGE

🚀 All packages built successfully!

- Run the CLI with `make run-cli`
- The HTML resume has been output to $(WEB_DIR)/resume.html
- The PDF resume has been output to $(WEB_DIR)/resume.pdf
endef

copy-files:
	cp $(RESUME_JSON) $(CLI_DIR)/
	cp $(SCHEMA_JSON) $(CLI_DIR)/
	cp $(RESUME_JSON) $(WEB_DIR)/

# Build CLI package
build-cli: copy-files
	cd $(CLI_DIR) && cargo build --release

# Run the compiled CLI
run-cli: build-cli
	./$(CLI_DIR)/target/release/resume-cli

# Build web package
build-web: copy-files
	cd $(WEB_DIR) && bun install && bun run build && bun run pdf

# Build all packages
build: build-cli build-web
	$(info $(POST_BUILD_MESSAGE))

.PHONY: copy-files build-cli run-cli build-web build
