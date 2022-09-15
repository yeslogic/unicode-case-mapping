tables:
	yeslogic-ucd-generate case-folding-simple --rust-match ../ucd-generate/ucd-15.0.0/ > src/case_folding_simple.rs
	yeslogic-ucd-generate case-mapping ../ucd-generate/ucd-15.0.0/ > src/tables.rs


.PHONY: tables

