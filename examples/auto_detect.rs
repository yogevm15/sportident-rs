use sportident::Reader;

#[tokio::main]
async fn main() {
    let mut reader = Reader::auto_connect().await.expect("failed to connect");
    loop {
        let card_data = reader.poll_owner_data().await.expect("failed to poll card");

        println!("{:?}", card_data);

        reader
            .beep_until_card_removed()
            .await
            .expect("failed to beep");
    }
}
