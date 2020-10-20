# hash-gen

hash-gen is a simple project I built over the course of a few days to practice using external Rust crates and improve my understanding of ownership.
hash-gen will return the MD5, SHA3 (256), or Blake2 hash of a file given via command line.
to build it yourself locally, try:

	git clone https://github.com/jfarabee/hash-gen
	cd hash-gen
	cargo build

it's that easy! to run, change into the 'debug' directory, and follow the below.

## Usage

	./hash-gen [OPTION] [FILEPATH]

Options:

	MD5
	SHA3
	Blake2

Filepath can be relative or absolute.
