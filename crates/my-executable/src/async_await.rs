use smol::{io, net, prelude::*, Unblock};

pub fn main() -> io::Result<()> {
    smol::block_on(async {
        let mut stream = net::TcpStream::connect("example.com:80").await?;

        let mut stream = match net::TcpStream::connect("example.com:80").await {
            Ok(stream) => stream,
            Err(error) => return Err(error),
        };

        let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await?;

        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(stream, &mut stdout).await?;
        Ok(())
    })
}
