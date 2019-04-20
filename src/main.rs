use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use byteorder::{ByteOrder, LittleEndian};

fn main() -> io::Result<()> {
    let f = File::open("daq.log")?;
    let mut float_buf = [0;24]; 
    let mut buf_reader = BufReader::new(f);
    let mut done = false;
    loop{
        match buf_reader.read_exact(&mut float_buf) {
            Ok(o) => o,
            Err(_e) => done = true,
        };
        let ax = LittleEndian::read_f32(&float_buf[0 .. 4])/100.0;
        let ay = LittleEndian::read_f32(&float_buf[4 .. 8])/100.0;
        let az = LittleEndian::read_f32(&float_buf[8 .. 12])/100.0;
        let drow = LittleEndian::read_f32(&float_buf[12 .. 16])/100.0;
        let dpitch = LittleEndian::read_f32(&float_buf[16 .. 20])/100.0;
        let dyaw = LittleEndian::read_f32(&float_buf[20 .. 24])/100.0;
    
        println!("Ax={:.2} Ay={:.2} Az={:.2} dRow={:.2} dPitch={:.2} dYaw={:.2}",ax,ay,az,drow,dpitch,dyaw);
        if done == true{
            break;
        }
    }
    Ok(())
}
