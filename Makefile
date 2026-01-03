.PHONY: bump

bump:
	@if [ -z "$(version)" ]; then echo "Usage: make bump version=X.Y.Z"; exit 1; fi
	$(eval V := $(shell echo $(version) | sed 's/^v//'))
	@echo "Bumping version to $(V)"
	@echo $(V) > VERSION
	@sed -i 's/^version = .*/version = "$(V)"/' Cargo.toml
	@cargo update -p rime --precise $(V) 2>/dev/null || true
