use sportident::Reader;

#[tokio::main]
async fn main() {
    let card_data = Reader::connect("/dev/ttyUSB0")
        .await
        .expect("failed to connect")
        .poll_card()
        .await
        .expect("failed to poll card");

    // print card data.
    dbg!(card_data);
}
