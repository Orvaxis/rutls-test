mod aws_lc_tls;
mod ring_tls;

use std::time::Instant;
use tokio;

/// Performance test function
async fn test_tls_providers(
    address: &str,
    server_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TLS Provider Performance Test ===");
    println!("Target: {}", address);
    println!("Server: {}", server_name);
    println!();

    // Test AWS-LC-RS
    println!("1. AWS-LC-RS:");
    let start_time = Instant::now();
    match aws_lc_tls::connect_aws_lc_tls(address, server_name.to_string()).await {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            println!("[AWS-LC-RS] Success, time: {:?}", elapsed);
        }
        Err(e) => println!("[AWS-LC-RS] Failed: {}", e),
    }

    println!();

    // Test Ring
    println!("2. Ring:");
    let start_time = Instant::now();
    match ring_tls::connect_ring_tls(address, server_name.to_string()).await {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            println!("[Ring] Success, time: {:?}", elapsed);
        }
        Err(e) => println!("[Ring] Failed: {}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rustls TLS Provider Comparison Test");
    println!("Testing fixed cipher suite configuration");
    println!();

    // Performance test
    test_tls_providers("120.53.53.53:443", "dns.pub").await?;

    Ok(())
}
