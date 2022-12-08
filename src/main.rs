use liveavionics::handler::run;



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    
    let resp = run().await;

    print!("{}", resp.unwrap());

    Ok(())

}


