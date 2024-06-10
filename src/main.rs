use std::error::Error;
use std::io;

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort, PortInfoError, SendError};

mod errors;

fn get_out_ports_with_names(midi_out: &MidiOutput) -> Result<Vec<(String, MidiOutputPort)>, PortInfoError> {
    let out_ports = midi_out.ports();
    let out_ports: Vec<_> = out_ports.into_iter()
        .map(|out_port| -> Result<(String, MidiOutputPort), PortInfoError> {
            let name = midi_out.port_name(&out_port)?;
            Ok((name, out_port))
        }).collect::<Result<_, _>>()?;
    Ok(out_ports)
}

fn select_output_port(out_ports: &[(String, MidiOutputPort)]) -> Result<&MidiOutputPort, Box<dyn Error>> {
    if out_ports.len() == 0 {
        return Err(errors::NoMidiOutputsError.into());
    }
    println!("Available MIDI device output ports:");
    for (i, (out_port_name, _)) in out_ports.iter().enumerate() {
        println!("{} - {}", i + 1, out_port_name);
    }
    let stdin = io::stdin();
    let err_message = "Input was not a valid choice; try again.";
    loop {
        println!("Type a number to select a MIDI device output port:");
        let mut input_raw = String::new();
        stdin.read_line(&mut input_raw)?;
        let input_int = match input_raw.trim().parse::<usize>() {
            Ok(input_int) => {input_int}
            Err(_) => {
                println!("{err_message}");
                continue;
            }
        };
        if input_int < 1 || input_int > out_ports.len() {
            println!("{err_message}");
            continue;
        }
        let (selected_port_name, selected_port) = out_ports.get(input_int - 1)
            .expect("Invalid index for selecting MIDI device output port");
        println!("Selected MIDI device output port '{selected_port_name}'");
        return Ok(selected_port);
    }
}

fn send_messages(conn_out: &mut MidiOutputConnection) -> Result<(), SendError> {
    println!("Sending some sysex messages to the MIDI output connection...");
    conn_out.send(&[240, 0, 0, 102, 20, 114, 6, 6, 6, 6, 6, 6, 6, 6, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 56, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 63, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 70, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 77, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 84, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 91, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 98, 80, 97, 110, 32, 32, 32, 32, 247])?;
    conn_out.send(&[240, 0, 0, 102, 20, 18, 105, 80, 97, 110, 32, 32, 32, 32, 247])?;
    println!("Done sending sysex messages");
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("output")?;
    let out_ports_with_names = get_out_ports_with_names(&midi_out)?;
    let out_port = select_output_port(&out_ports_with_names)?;
    let mut conn_out = midi_out.connect(&out_port, "test port")?;
    let result = send_messages(&mut conn_out);
    conn_out.close();
    Ok(result?)
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {e}");
        }
    };
}
