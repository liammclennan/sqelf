use crate::support::*;

pub fn test() {
    let mut server = server::tcp();
    let mut stream = tcp::stream();

    stream.write(net_chunks![
        ..net_chunks!({
            "host": "foo",
            "short_message": "bar"
        }),
        ..tcp_delim()
    ]);

    server.receive(|received| {
        assert_eq!("bar", received["@m"]);
    });

    assert_eq!(1, server.received());

    stream.close();
    server.close();
}
