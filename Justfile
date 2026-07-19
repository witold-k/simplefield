user_name        := env("USER")
current_location := justfile()
current_dir      := justfile_directory()
module_name      := file_name(current_dir)

default: build

build:
    cargo build
    RUST_BACKTRACE=1 cargo test
    cargo clippy

fix:
    @RUST_BACKTRACE=1 aifix -l rust -t fix_code -f {{current_dir}} -f {{current_dir}}/..

fixd:
    @RUST_BACKTRACE=1 aifix -d -l rust -t fix_code -f {{current_dir}} -f {{current_dir}}/..

fixws:
    @RUST_BACKTRACE=1 aifix -l rust -t fix_code -w ~/svn/_workspace -f {{current_dir}} -f {{current_dir}}/.. -f ~/svn/_workspace

review:
    @RUST_BACKTRACE=1 aifix -l rust -t review_code -w ~/svn/_workspace -f {{current_dir}} -f {{current_dir}}/.. -f ~/svn/_workspace

reviewd:
    @RUST_BACKTRACE=1 aifix -d -l rust -t review_code -w ~/svn/_workspace -f {{current_dir}} -f {{current_dir}}/.. -f ~/svn/_workspace

doc_item:
    RUST_BACKTRACE=1 aifix -l rust -t write_item_doc -f {{current_dir}} -f {{current_dir}}/..

doc_itemd:
    RUST_BACKTRACE=1 aifix -d -l rust -t write_item_doc -f {{current_dir}} -f {{current_dir}}/..

docws_item:
    RUST_BACKTRACE=1 aifix -l rust -t write_item_doc -w ~/svn/_workspace -f {{current_dir}} -f {{current_dir}}/..

doc_block:
    RUST_BACKTRACE=1 aifix -l rust -t write_block_doc -f {{current_dir}} -f {{current_dir}}/..

doc_blockd:
    RUST_BACKTRACE=1 aifix -d -l rust -t write_block_doc -f {{current_dir}} -f {{current_dir}}/..

doc_module:
    RUST_BACKTRACE=1 aifix -l rust -t write_module_doc -f {{current_dir}} -f {{current_dir}}/..

doc_moduled:
    RUST_BACKTRACE=1 aifix -d -l rust -t write_module_doc -f {{current_dir}} -f {{current_dir}}/..

doc:
    aifix -t doc_code -f {{current_dir}} -f {{current_dir}}/..

clean:
    @cargo clean -p {{module_name}}

clean-all:
    @rm target -rf
    @cargo clean

targetlist:
    rustup target list

rpi:
    cargo build --target aarch64-unknown-linux-gnu

cover:
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/cargo-test-%p-%m.profraw' cargo test
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
    firefox target/coverage/html/index.html

keygen:
    gpg --full-generate-key

keyexport:
    gpg --armor --export -a Witold > gpg.pub
    gpg --armor --export-secret-keys -a Witold > gpg.priv

keyimport:
    gpg --import gpg.pub
    gpg --import gpg.priv

# github.com/spwhitton/git-remote-gcrypt
addsecret:
    git remote add cryptremote gcrypt::rsync://vsrv/home/{{user_name}}/depots/{{module_name}}

removesecret:
    git remote remove cryptremote

helpsecret:
    gpg --list-keys

