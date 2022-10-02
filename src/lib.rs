use alsa::seq::{
    ClientIter,
    Seq,
};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn list_controllers() -> Result<(), ()> {
    let seq = Seq::open(None, None, false).unwrap();
    for client in ClientIter::new(&seq) {
        println!("{}\t{}", client.get_client(), client.get_name().unwrap_or("unknown"));
    }
    Ok(())
}
