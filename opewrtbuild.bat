
docker run --rm --privileged=true -v /d/rust/ddnsv6-rust/src:/home/rust/src messense/rust-musl-cross:mipsel-musl  cargo build --release