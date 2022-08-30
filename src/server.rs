use tonic::{transport::Server, Request, Response, Status};

pub mod nth_odd {
    tonic::include_proto!("nthodd");
}

use nth_odd::calculator_server::{Calculator, CalculatorServer};
use nth_odd::{NthOddReply, NthOddRequest};

fn find_nth_odd(n: u32) -> u32 {
    let mut i = 0;
    let mut odd_count = 0;

    while odd_count != n {
        i += 1;
        if i % 2 == 1 {
            odd_count += 1;
        }
    }

    i
}

#[derive(Debug, Default)]
pub struct MyCalculator {}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn find_nth_odd(
        &self,
        request: Request<NthOddRequest>,
    ) -> Result<Response<NthOddReply>, Status> {
        let nth_odd = find_nth_odd(request.into_inner().value);

        let reply = NthOddReply { value: nth_odd };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyCalculator::default();

    Server::builder()
        .add_service(CalculatorServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
