use rustls::crypto::CryptoProvider;
use rustls::crypto::ring;
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

/// Create Ring based TLS client configuration with specific cipher suites
pub fn create_ring_tls_config() -> Result<Arc<ClientConfig>, Box<dyn std::error::Error>> {
    let mut root_store = RootCertStore::empty();

    // Add system root certificates
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    // Create provider with specific cipher suites
    let provider = CryptoProvider {
        ..ring::default_provider()
    };

    let config = ClientConfig::builder_with_provider(provider.into())
        .with_safe_default_protocol_versions()?
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(Arc::new(config))
}

/// Connect to specified address using Ring
pub async fn connect_ring_tls(
    address: &str,
    server_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Ring] Connecting to {}...", address);

    // Create TCP connection
    let tcp_stream = TcpStream::connect(address)?;

    // Create Ring based TLS configuration
    let config = create_ring_tls_config()?;

    // Create TLS connection
    let server_name = rustls::pki_types::ServerName::try_from(server_name)?;
    let tls_connection = ClientConnection::new(config, server_name)?;

    // Create TLS stream
    let mut tls_stream = StreamOwned::new(tls_connection, tcp_stream);

    println!("[Ring] TLS handshake successful!");

    // Display negotiated cipher suite
    if let Some(suite) = tls_stream.conn.negotiated_cipher_suite() {
        println!("[Ring] Negotiated cipher suite: {:?}", suite.suite());
    }

    // Send simple HTTP request
    let request = "GET / HTTP/1.1\r\nHost: dns.pub\r\nConnection: close\r\n\r\n";
    tls_stream.write_all(request.as_bytes())?;

    // Read response
    let mut response = String::new();
    tls_stream.read_to_string(&mut response)?;

    println!("[Ring] Response received:");
    println!("{}", response);

    Ok(())
}
