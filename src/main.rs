use std::error::Error;
use std::io;
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort, MidiOutputPorts, SendError};

fn select_output_port<'a>(midi_out: &MidiOutput, out_ports: &'a MidiOutputPorts) -> Result<&'a MidiOutputPort, Box<dyn Error>> {
    if out_ports.len() == 0 {
        return Err("No MIDI device output ports found!".into());
    }
    println!("Available MIDI device output ports:");
    for (i, out_port) in out_ports.iter().enumerate() {
        let out_port_name = midi_out.port_name(&out_port)?;
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
        let selected_port = out_ports.get(input_int - 1).expect("Invalid index for selecting MIDI device output port");
        println!("Selected MIDI device output port '{}'", midi_out.port_name(&selected_port)?);
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

fn main() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("output")?;
    let out_ports = midi_out.ports();
    let out_port = select_output_port(&midi_out, &out_ports)?;
    let mut conn_out = midi_out.connect(&out_port, "test port")?;
    let result = send_messages(&mut conn_out);
    conn_out.close();
    Ok(result?)
}
