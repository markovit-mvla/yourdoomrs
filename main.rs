use std::io::{self, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let host = "irc.freenode.net:6667";
    let tcp_stream = TcpStream::connect(host)?;
    let mut writer = io::BufWriter::new(tcp_stream);
    let payload = 
        b"USER yourdoom 0 * :realname\nNICK yourdoom\nJOIN #yourchannel\nPRIVMSG #yourchannel :Doomed! (Rust style) \n";
    writer.write_all(payload)?;
    writer.flush()?;
    Ok(())
}
