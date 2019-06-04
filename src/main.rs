use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

fn main() -> io::Result<()> {
    let file_name = "daq_imu_pkpk60_0_0_0.2hz.log";
    let f = File::open(file_name)?;
    let mut buf_reader = BufReader::new(f);

    let wf = File::create(format!("{}.csv",file_name))?;
    let mut buf_writer = BufWriter::new(wf);

    let mut float_buf = [0; 36];
    let mut done = false;

    writeln!(&mut buf_writer, "ax;ay;az;d_row;d_pitch;d_yaw;mX;mY;mZ")?;

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
        let mx = LittleEndian::read_f32(&float_buf[24..28]) / 100.0;
        let my = LittleEndian::read_f32(&float_buf[28..32]) / 100.0;
        let mz = LittleEndian::read_f32(&float_buf[32..36]) / 100.0;

        println!(
            "Ax={:.5} Ay={:.5} Az={:.5} dRow={:.5} dPitch={:.5} dYaw={:.5} mX={:.5} mY={:.5} mZ={:.5}",
            ax, ay, az, drow, dpitch, dyaw, mx, my, mz
        );
        writeln!(
            &mut buf_writer,
            "{:.5};{:.5};{:.5};{:.5};{:.5};{:.5};{:.5};{:.5};{:.5}",
            ax, ay, az, drow, dpitch, dyaw, mx, my, mz
        )?;
    }
    Ok(())
}
