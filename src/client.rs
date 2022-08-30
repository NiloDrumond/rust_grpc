pub mod nth_odd {
    tonic::include_proto!("nthodd");
}

use nth_odd::calculator_client::CalculatorClient;
use nth_odd::NthOddRequest;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;

    let num: u32 = 10_000_000;
    let mut durations = [0; 10_000];

    let mut i = 0;

    while i < durations.len() {
        let now = Instant::now();
        let request = tonic::Request::new(NthOddRequest { value: num });
        let _response = client.find_nth_odd(request).await;
        durations[i] = now.elapsed().as_millis() as i32;
        i += 1;
    }

    let mean_duration: i32 = durations.iter().sum::<i32>() / durations.len() as i32;
    let variance: f32 = durations
        .iter()
        .map(|value| {
            let diff: i32 = mean_duration - value;
            diff * diff
        })
        .sum::<i32>() as f32
        / durations.len() as f32;
    let std_deviation = variance.sqrt();

    println!("Average RTT: {}", mean_duration);
    println!("Standard Deviation: {}", std_deviation);

    Ok(())
}
