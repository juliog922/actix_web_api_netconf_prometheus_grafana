use ssh2::{Channel, Session};
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

const HELLO: &str = "<hello xmlns=\"urn:ietf:params:xml:ns:netconf:base:1.0\">
  <capabilities>
    <capability>urn:ietf:params:netconf:base:1.1</capability>
  </capabilities>
</hello>
]]>]]>";

/// Reads data from the given SSH channel until a specific end sequence is encountered.
///
/// # Arguments
///
/// * `channel` - A mutable reference to the SSH channel from which to read data.
///
/// # Returns
///
/// A `Result` containing the read data as a `String` or an `Error` if reading fails.
fn read(channel: &mut Channel) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    loop {
        // Reading 1 byte at a time is inefficient; this is for example purposes only.
        let mut buffer = [1u8; 1];
        let bytes_read = channel.read(&mut buffer[..])?;
        let s = String::from_utf8_lossy(&buffer[..bytes_read]);
        result.push_str(&s);
        if result.ends_with("]]>]]>") {
            break;
        }
        if result.ends_with("##") {
            break;
        }
        if bytes_read == 0 || channel.eof() {
            println!("Buffer is empty, SSH channel read terminated");
            break;
        }
    }
    Ok(result)
}

/// Establishes an SSH connection to a host and sends a NETCONF payload.
///
/// # Arguments
///
/// * `host` - The hostname or IP address of the target device.
/// * `port` - The port number for the SSH connection.
/// * `username` - The username for SSH authentication.
/// * `password` - The password for SSH authentication.
/// * `payload` - The NETCONF payload to be sent.
///
/// # Returns
///
/// A `Result` containing the response from the device as a `String` or an `Error` if the operation fails.
pub fn get(
    host: &str,
    port: isize,
    username: &str,
    password: &str,
    payload: &str,
) -> Result<String, Box<dyn Error>> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(username, password)?;
    let mut channel = sess.channel_session()?;
    channel.subsystem("netconf")?;
    let _ = read(&mut channel)?;
    println!("Connected");

    let payload = format!("{}\n#{}\n{}\n##\n", HELLO, payload.len(), payload);
    let _ = channel.write(payload.as_bytes())?;
    let result = read(&mut channel)?;

    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;
    Ok(result)
}
