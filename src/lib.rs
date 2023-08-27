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

use directories_next::ProjectDirs;

use regex::Regex;

use serde_json::Value;

use std::{
    collections::HashMap,
    ffi::CString,
    fs::File,
    io::BufReader,
    process::Command,
};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn get_directories() -> ProjectDirs {
    ProjectDirs::from("net", "Arkanosis", "smhkd").unwrap()
}

fn load_config() -> Value {
    let directories = get_directories();
    let config_directory = directories.config_dir();
    let mut config_path = config_directory.to_owned();
    // TODO handle the non-json, sxhkdrc-like configuration file as well
    config_path.push("smhkdrc.json");
    if let Ok(config_file) = File::open(&config_path) {
        let config_reader = BufReader::new(config_file);
        serde_json::from_reader(config_reader).unwrap()
    } else {
        Value::default()
    }
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
    // TODO reload configuration on SIGUSR1
    let config = load_config();

    let seq = Seq::open(None, Some(Direction::Capture), false).unwrap();

    let device_name = CString::new("smhkd").unwrap();
    seq.set_client_name(&device_name).unwrap();

    let mut port_info = PortInfo::empty().unwrap();
    port_info.set_name(&device_name);
    port_info.set_capability(PortCap::WRITE | PortCap::SUBS_WRITE);
    port_info.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    seq.create_port(&port_info).unwrap();

    let value_pattern = Regex::new(r"\$VALUE\b").unwrap();

    let mut subscribers: HashMap<String, &Value> = HashMap::new();

    if let Some(clients) = config.as_object() {
        for client_name in clients.keys() {
            let mut client = 0;
            let mut port = 0;
            if client_name.starts_with('@') {
                let seq = Seq::open(None, None, false).unwrap();
                for available_client in ClientIter::new(&seq) {
                    if available_client.get_name().unwrap() == &client_name[1..] {
                        client = available_client.get_client();
                        for available_port in PortIter::new(&seq, available_client.get_client()) {
                            port = available_port.get_port();
                            // TODO don't break on first port if port is specified in config
                            break;
                        }
                    }
                }
                // TODO warn at least if not found
            } else {
                if let Some((client_string, port_string)) = client_name.split_once(":") {
                    client = client_string.parse().unwrap();
                    port = port_string.parse().unwrap();
                } else {
                    // TODO warn at least
                }
            }
            let subscription = PortSubscribe::empty().unwrap();
            subscription.set_sender(Addr {
                client,
                port,
            });
            subscription.set_dest(Addr {
                client: seq.client_id().unwrap(),
                port: port_info.get_port(),
            });
            seq.subscribe_port(&subscription).unwrap_or_else(|_| {
                eprintln!("smhkd: unable to subscribe to client {}, port {}", client, port);
            });

            subscribers.insert(format!("{}:{}", client, port), clients.get(client_name).unwrap());
        }

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
                    let event_source = event.get_source();
                    if let Some(controllers) = subscribers.get(&format!("{}:{}", event_source.client, event_source.port)) {
                        let event_data: EvCtrl = event.get_data().unwrap();
                        let command_template = controllers.get(&format!("{}", event_data.param)).unwrap().as_str().unwrap();
                        // TODO consider using a shell here
                        //   pros:
                        //     - $VALUE can be passed as an environment variable
                        //     - other environment variables can be used
                        //     - argument splitting and other shell features are builtin
                        //   cons:
                        //     - security risks (only if the user is unaware)
                        //     - performance overhead of double fork/exec (mitigated by providing builtins for most common use-cases that don't even fork)
                        let command = value_pattern.replace_all(command_template, format!("{}", event_data.value));
                        let mut command_parts: Vec<&str> = command.split(' ').collect();
                        let (program, arguments) = (command_parts.remove(0), command_parts);
                        Command::new(program)
                            .args(arguments)
                            .status()
                            .unwrap();
                    }
                }
            }
            poll(&mut descriptors, -1).unwrap();
        }
    }
}
