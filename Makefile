RUST_DIR = src/
RUST_FILES = main.rs
RUST_SRC = $(addprefix $(RUST_DIR), $(RUST_FILES))

NAME = ft_lex

RUST_PATH = $$HOME/.cargo/bin/

all: install-rust ${NAME}

install-rust:
ifeq ($(shell command -v ${RUST_PATH}rustc && echo yes || echo no), no)
	@echo "rust is not installed. Installing rust..."
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y;
endif


${NAME}: ${RUST_SRC}
	rustc src/main.rs -o ${NAME}

clean:
	@${RUST_PATH}cargo clean > /dev/null 2>&1;

fclean: clean
	@rm -f ${NAME}  Cargo.lock

re: fclean ${NAME}

.PHONY: all clean fclean re install-rust