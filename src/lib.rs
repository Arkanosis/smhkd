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

use std::{
    ffi::CString,
    process::Command,
};

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
    let seq = Seq::open(None, Some(Direction::Capture), false).unwrap();

    let device_name = CString::new("smhkd").unwrap();
    seq.set_client_name(&device_name).unwrap();

    let mut port_info = PortInfo::empty().unwrap();
    port_info.set_name(&device_name);
    port_info.set_capability(PortCap::WRITE | PortCap::SUBS_WRITE);
    port_info.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    seq.create_port(&port_info).unwrap();

    // TODO subscribe to controllers from configuration
    // TODO reload configuration on SIGUSR1
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
        loop {
            let event_pending = input.event_input_pending(true).unwrap();
            if event_pending == 0 {
                break;
            }
            let event = input.event_input().unwrap();
            if event.get_type() == EventType::Controller {
                let event_data: EvCtrl = event.get_data().unwrap();
                // TODO run commands according to configuration
                match event_data.param {
                    0 => {
                        Command::new("pactl")
                            .arg("set-sink-volume")
                            .arg("@DEFAULT_SINK@")
                            .arg(format!("{}%", event_data.value))
                            .status()
                            .unwrap();
                    }
                    1 => {
                        Command::new("pactl")
                            .arg("set-sink-input-volume")
                            .arg("47518") // TODO FIXME pactl list sinks short
                            .arg(format!("{}%", event_data.value))
                            .status()
                            .unwrap();
                    }
                    32 => {
                        Command::new("pactl")
                            .arg("set-sink-volume")
                            .arg("@DEFAULT_SINK@")
                            .arg("100%")
                            .status()
                            .unwrap();
                    }
                    33 => {
                        Command::new("pactl")
                            .arg("set-sink-input-volume")
                            .arg("47518") // TODO FIXME pactl list sinks short
                            .arg("100%")
                            .status()
                            .unwrap();
                    }
                    48 => {
                        Command::new("pactl")
                            .arg("set-sink-volume")
                            .arg("@DEFAULT_SINK@")
                            .arg("30%")
                            .status()
                            .unwrap();
                    }
                    49 => {
                        Command::new("pactl")
                            .arg("set-sink-input-volume")
                            .arg("47518") // TODO FIXME pactl list sinks short
                            .arg("80%")
                            .status()
                            .unwrap();
                    }
                    64 => {
                        Command::new("pactl")
                            .arg("set-sink-volume")
                            .arg("@DEFAULT_SINK@")
                            .arg("0%")
                            .status()
                            .unwrap();
                    }
                    65 => {
                        Command::new("pactl")
                            .arg("set-sink-input-volume")
                            .arg("47518") // TODO FIXME pactl list sinks short
                            .arg("0%")
                            .status()
                            .unwrap();
                    }
                    _ => ()
                }

            }
        }
        poll(&mut descriptors, -1).unwrap();
    }
}
