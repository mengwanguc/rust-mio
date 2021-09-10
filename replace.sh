sed -i '/pub\sfn\sobject_read()/i \
}\
#[link(name = "mio")]\
#[link(name = "motr")]\
extern "C" {\
    ' target/debug/build/rust-mio-4ee34cf8ec502600/out/bindings.rs
