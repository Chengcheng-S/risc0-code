SHELL=/bin/bash

all:
	@make help


.PHONY: help ## Display help commands
help:
	@printf 'Usage:\n'
	@printf '  make <tagert>\n'
	@printf '\n'
	@printf 'Targets:\n'
	@IFS=$$'\n' ; \
    help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//'`); \
    for help_line in $${help_lines[@]}; do \
        IFS=$$'#' ; \
        help_split=($$help_line) ; \
        help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		IFS=$$':' ; \
		phony_command=($$help_split); \
        help_command=`echo $${phony_command[1]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		printf "  %-50s %s\n" $$help_command $$help_info ; \
    done

# format
.PHONY: fmtcheck ## cargo fmt check
fmtcheck:
	cargo fmt --all -- --check

.PHONY: taplocheck ## taplo fmt check
#  cargo install taplo-cli --locked
taplocheck:
	taplo fmt --check

.PHONY: fmt ## cargo fmt all && taplo fmt all
fmt:
	cargo fmt --all && taplo fmt

.PHONY: clean
clean :	
	cargo clean


.PHONY : pprof ## generate pprof file
pprof :
	RISC0_PPROF_OUT=./profile.pb cargo run

.PHONY : pprof-http ## serve pprof file
pprof-http:
	go tool pprof -http=127.0.0.1:8000 profile.pb