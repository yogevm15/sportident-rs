use std::str::FromStr;

use chrono::NaiveTime;

use crate::protocol::punch::DayOfWeek::Friday;
use crate::protocol::punch::StartOrFinishPunch::{Normal, SubSecond};
use crate::protocol::punch::WeekCounter::{First, Second, Third};
use crate::protocol::punch::{Punch, SubSecondPunch};
use crate::protocol::responses::card::CardType::Si8;
use crate::protocol::{CardOwnerData, CardReadout, FromCardBlocks};

#[tokio::test]
async fn empty() {
    let mut data = hex::decode("05760c87eaeaeaea1a01a02eeeeeeeeeeeeeeeee000000b3021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: None, finish: None, check: Some(Punch { time: NaiveTime::from_str("11:23:26").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn only_finish() {
    let mut data = hex::decode("05760c87eaeaeaea1a01a2b2eeeeeeee1a01a2b7000000b3021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: None, finish: Some(Normal(Punch { time: NaiveTime::from_str("11:34:15").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("11:34:10").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn start_finish() {
    let mut data = hex::decode("05760c87eaeaeaea1a01a23a2a01a2331a01a244000000b3021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: Some(Normal(Punch { time: NaiveTime::from_str("11:32:03").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: Some(Normal(Punch { time: NaiveTime::from_str("11:32:20").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("11:32:10").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn full() {
    let mut data = hex::decode("05760c87eaeaeaea1a01a3422a01a3441a01a37b00431e42021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a3fa34d2a43a34e2a3fa34f2a43a34f2a3fa3502a43a3512a3fa3512a43a3522a3fa3522a43a3532a3fa3532a43a3542a3fa3542a43a3552a3fa3562a43a3562a3fa3572a43a3582a3fa3592a43a3592a3fa35a2a43a35a2a3fa35b2a43a35c2a3fa35d2a43a35e2a3fa35e2a43a35f2a3fa35f2a43a360").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: Some(Normal(Punch { time: NaiveTime::from_str("11:36:36").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: Some(Normal(Punch { time: NaiveTime::from_str("11:37:31").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("11:36:34").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![Punch { time: NaiveTime::from_str("11:36:45").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:46").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:47").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:47").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:48").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:49").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:49").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:50").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:50").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:51").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:51").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:52").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:52").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:53").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:54").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:54").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:55").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:56").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:57").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:57").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:58").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:36:58").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:36:59").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:37:00").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:37:01").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:37:02").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:37:02").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:37:03").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:37:03").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:37:04").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }] };

    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn half_full() {
    let mut data = hex::decode("05760c87eaeaeaea1a01a65e2a01a6571a01a670003f0f88021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a3fa6592a43a65a2a3fa65b2a43a65c2a3fa65c2a43a65d2a3fa65e2a43a65e2a3fa65f2a43a6602a3fa6612a43a6622a3fa6622a43a6632a3fa664eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: Some(Normal(Punch { time: NaiveTime::from_str("11:49:43").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: Some(Normal(Punch { time: NaiveTime::from_str("11:50:08").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("11:49:50").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![Punch { time: NaiveTime::from_str("11:49:45").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:46").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:47").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:48").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:48").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:49").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:50").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:50").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:51").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:52").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:53").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:54").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:54").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("11:49:55").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("11:49:56").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn only_start() {
    let mut data = hex::decode("05760c87eaeaeaea1a01a69e2a01a699eeeeeeee000000b3021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: Some(Normal(Punch { time: NaiveTime::from_str("11:50:49").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: None, check: Some(Punch { time: NaiveTime::from_str("11:50:54").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn sub_second() {
    let mut data = hex::decode("05760c87eaeaeaea1b015ac08b1b5abb8bd85abc000000b3021f9b2a0cffcbfe4461666e613b596f6765763b4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 2071338, card_type: Si8, start: Some(SubSecond(SubSecondPunch { time: NaiveTime::from_str("18:27:07.105").unwrap(), day_of_week: Friday, week_counter: First })), finish: Some(SubSecond(SubSecondPunch { time: NaiveTime::from_str("18:27:08.847").unwrap(), day_of_week: Friday, week_counter: First })), check: Some(Punch { time: NaiveTime::from_str("18:27:12").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, Si8).await.unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn empty_owner_data() {
    let mut data = hex::decode("05760c87eaeaeaea1b015ac08b1b5abb8bd85abc000000b3021f9b2a0cffcbfe3b3beeee613b596f6765763b4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardOwnerData { first_name: "".to_string(), last_name: "".to_string(), gender: None, birthday: None, club: None, email: None, phone: None, city: None, street: None, zip: None, country: None };
    assert_eq!(
        CardOwnerData::from_card_blocks(&mut data, Si8)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn normal_owner_data() {
    let mut data = hex::decode("05760c87eaeaeaea1d01582ceeeeeeeeeeeeeeee001f0b07021f9b2a0cffcbfe746573743b746573743beeee4150484e413b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f0372001f03e8001f03ee0d1f595f0d1f59650d1f596b0d1f59710d1f597a0d1f59860d1f598e0d1f5992eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardOwnerData { first_name: "test".to_string(), last_name: "test".to_string(), gender: None, birthday: None, club: None, email: None, phone: None, city: None, street: None, zip: None, country: None };
    assert_eq!(
        CardOwnerData::from_card_blocks(&mut data, Si8)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn full_owner_data() {
    let mut data = hex::decode("05760c87eaeaeaea1d01582ceeeeeeeeeeeeeeee001f0b07021f9b2a0cffcbfe74657374746573747465733b7465737474657374746573743beeeeee00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f0372001f03e8001f03ee0d1f595f0d1f59650d1f596b0d1f59710d1f597a0d1f59860d1f598e0d1f5992eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardOwnerData { first_name: "testtesttes".to_string(), last_name: "testtesttest".to_string(), gender: None, birthday: None, club: None, email: None, phone: None, city: None, street: None, zip: None, country: None };
    assert_eq!(
        CardOwnerData::from_card_blocks(&mut data, Si8)
            .await
            .unwrap(),
        expected_data
    );
}
