pub mod aws_lc_tls;
pub mod ring_tls;

use std::time::Instant;
use tokio;

/// Performance test function
async fn benchmark_tls_providers(
    address: &str,
    server_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TLS Provider Test ===");
    println!("Target: {}", address);
    println!();

    // Test AWS-LC-RS
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
