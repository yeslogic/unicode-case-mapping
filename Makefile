tables:
	yeslogic-ucd-generate case-folding-simple --rust-match ../ucd-generate/ucd-16.0.0/ > src/case_folding_simple.rs
	yeslogic-ucd-generate case-mapping ../ucd-generate/ucd-16.0.0/ > src/tables.rs
	cargo fmt
	sed -i.bak -E '/\(([0-9]+), &\[\1\]\)/d' src/tables.rs
	rm src/tables.rs.bak


.PHONY: tables

