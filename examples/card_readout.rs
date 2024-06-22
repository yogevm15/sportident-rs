use sportident::Reader;

#[tokio::main]
async fn main() {
    let mut reader = Reader::auto_connect().await.expect("failed to connect");
    loop {
        reader
            .beep_until_card_removed()
            .await
            .expect("failed to beep");

        let card_data = reader.poll_card().await.expect("failed to poll card");

        println!("{:?}", card_data);
    }
}
