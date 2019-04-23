use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

fn main() -> io::Result<()> {
    let f = File::open("daq_imu_0_pkpk35_0.2hz.log")?;
    let mut buf_reader = BufReader::new(f);

    let wf = File::create("out.csv")?;
    let mut buf_writer = BufWriter::new(wf);

    let mut float_buf = [0; 24];
    let mut done = false;

    writeln!(&mut buf_writer, "ax;ay;az;d_row;d_pitch;d_yaw")?;

    while done == false {
        match buf_reader.read_exact(&mut float_buf) {
            Ok(o) => o,
            Err(_e) => done = true,
        };
        let ax = LittleEndian::read_f32(&float_buf[0..4]) / 100.0;
        let ay = LittleEndian::read_f32(&float_buf[4..8]) / 100.0;
        let az = LittleEndian::read_f32(&float_buf[8..12]) / 100.0;
        let drow = LittleEndian::read_f32(&float_buf[12..16]) / 100.0;
        let dpitch = LittleEndian::read_f32(&float_buf[16..20]) / 100.0;
        let dyaw = LittleEndian::read_f32(&float_buf[20..24]) / 100.0;

        println!(
            "Ax={:.5} Ay={:.5} Az={:.5} dRow={:.5} dPitch={:.5} dYaw={:.5}",
            ax, ay, az, drow, dpitch, dyaw
        );
        writeln!(
            &mut buf_writer,
            "{:.5};{:.5};{:.5};{:.5};{:.5};{:.5}",
            ax, ay, az, drow, dpitch, dyaw
        )?;
    }
    Ok(())
}
