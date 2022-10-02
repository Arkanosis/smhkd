use alsa::{
    Direction,
    PollDescriptors,
    poll::poll,
    seq::{
        Addr,
        ClientIter,
        EvCtrl,
        EventType,
        PortCap,
        PortInfo,
        PortIter,
        PortSubscribe,
        PortType,
        Seq,
    },
};

use std::ffi::CString;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn list_controllers() -> () {
    let seq = Seq::open(None, None, false).unwrap();
    for client in ClientIter::new(&seq) {
        for port in PortIter::new(&seq, client.get_client()) {
            let capability = port.get_capability();
            if capability.contains(PortCap::READ | PortCap::SUBS_READ) {
                println!("{}:{}\t{}", client.get_client(), port.get_port(), client.get_name().unwrap_or("unknown"));
            }
        }
    }
}

pub fn run() -> () {
    let seq = Seq::open(None, Some(Direction::Capture), true).unwrap();

    let device_name = CString::new("smhkd").unwrap();
    seq.set_client_name(&device_name).unwrap();

    let mut port_info = PortInfo::empty().unwrap();
    port_info.set_name(&device_name);
    port_info.set_capability(PortCap::WRITE | PortCap::SUBS_WRITE);
    port_info.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    seq.create_port(&port_info).unwrap();

    // TODO subscribe to controllers from configuration
    let subscription = PortSubscribe::empty().unwrap();
    subscription.set_sender(Addr {
        client: 28,
        port: 0,
    });
    subscription.set_dest(Addr {
        client: seq.client_id().unwrap(),
        port: port_info.get_port(),
    });
    seq.subscribe_port(&subscription).unwrap();

    let mut descriptors = (&seq, Some(Direction::Capture)).get().unwrap();
    let mut input = seq.input();
    loop {
        let event_pending = input.event_input_pending(true).unwrap();
        if event_pending != 0 {
            let event = input.event_input().unwrap();
            if event.get_type() == EventType::Controller {
                let event_data: EvCtrl = event.get_data().unwrap();
                println!("Control change: {}, {}", event_data.param, event_data.value);
                // TODO run commands according to configuration
            }
        }
        poll(&mut descriptors, -1).unwrap();
    }
}
