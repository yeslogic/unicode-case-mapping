UCD:=16.0.0

# the sed command removes entries in the table that map to themselves, which
# makes the data tables a bit smaller.
tables:
	yeslogic-ucd-generate case-folding-simple --rust-match ../ucd-generate/ucd-$(UCD)/ > src/case_folding_simple.rs
	yeslogic-ucd-generate case-mapping ../ucd-generate/ucd-$(UCD)/ > src/tables.rs
	cargo fmt
	sed -i.bak -E '/\(([0-9]+), &\[\1\]\)/d' src/tables.rs
	rm src/tables.rs.bak


.PHONY: tables

