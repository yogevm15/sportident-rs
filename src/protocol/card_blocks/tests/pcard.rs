use std::str::FromStr;

use chrono::NaiveTime;

use crate::protocol::punch::DayOfWeek::Friday;
use crate::protocol::punch::StartOrFinishPunch::{Normal, SubSecond};
use crate::protocol::punch::WeekCounter::{First, Second, Third};
use crate::protocol::punch::{Punch, SubSecondPunch};
use crate::protocol::responses::card::CardType::PunchCard;
use crate::protocol::{CardOwnerData, CardReadout, FromCardBlocks};

#[tokio::test]
async fn empty() {
    let mut data = hex::decode("d962cc91eaeaeaea1b014e7aeeeeeeeeeeeeeeee000000890446cc050cff2dae343633393734393b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639749, card_type: PunchCard, start: None, finish: None, check: Some(Punch { time: NaiveTime::from_str("17:34:50").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn only_finish() {
    let mut data = hex::decode("d962cc91eaeaeaea1b014eb1eeeeeeee1b014eb6000000890446cc050cff2dae343633393734393b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639749, card_type: PunchCard, start: None, finish: Some(Normal(Punch { time: NaiveTime::from_str("17:35:50").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("17:35:45").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn start_finish() {
    let mut data = hex::decode("d962cc91eaeaeaea1b014ee32b014edc1b014ee7000000890446cc050cff2dae343633393734393b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639749, card_type: PunchCard, start: Some(Normal(Punch { time: NaiveTime::from_str("17:36:28").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: Some(Normal(Punch { time: NaiveTime::from_str("17:36:39").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("17:36:35").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn full() {
    let mut data = hex::decode("d962cc91eaeaeaea1b014be02b014bda1b014bfa004314c30446cc050cff2dae343633393734393b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002b3f4bdc2b434bdd2b3f4bdd2b434bde2b3f4bde2b434bdf2b3f4bdf2b434be02b3f4be02b434be12b3f4be12b434be22b3f4be22b434be32b3f4be32b434be42b3f4be52b434be52b3f4be62b434be6").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639749, card_type: PunchCard, start: Some(Normal(Punch { time: NaiveTime::from_str("17:23:38").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: Some(Normal(Punch { time: NaiveTime::from_str("17:24:10").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("17:23:44").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![Punch { time: NaiveTime::from_str("17:23:40").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:41").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:41").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:42").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:42").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:43").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:43").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:44").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:44").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:45").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:45").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:46").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:46").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:47").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:47").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:48").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:49").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:49").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:23:50").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:23:50").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }] };

    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn half_full() {
    let mut data = hex::decode("d962cc91eaeaeaea1b014f092b014f061b014f23003f0a610446cc050cff2dae343633393734393b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002b434f092b3f4f0b2b434f0d2b3f4f0f2b434f102b3f4f122b434f142b3f4f152b434f172b3f4f19eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639749, card_type: PunchCard, start: Some(Normal(Punch { time: NaiveTime::from_str("17:37:10").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: Some(Normal(Punch { time: NaiveTime::from_str("17:37:39").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 })), check: Some(Punch { time: NaiveTime::from_str("17:37:13").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![Punch { time: NaiveTime::from_str("17:37:13").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:37:15").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:37:17").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:37:19").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:37:20").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:37:22").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:37:24").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:37:25").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }, Punch { time: NaiveTime::from_str("17:37:27").unwrap(), day_of_week: Friday, week_counter: Third, code: 67 }, Punch { time: NaiveTime::from_str("17:37:29").unwrap(), day_of_week: Friday, week_counter: Third, code: 63 }] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn only_start() {
    let mut data = hex::decode("d962cc91eaeaeaea1b014f402b014f3beeeeeeee000000890446cc050cff2dae343633393734393b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639749, card_type: PunchCard, start: Some(Normal(Punch { time: NaiveTime::from_str("17:38:03").unwrap(), day_of_week: Friday, week_counter: Third, code: 1 })), finish: None, check: Some(Punch { time: NaiveTime::from_str("17:38:08").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn sub_second() {
    let mut data = hex::decode("7774cc91eaeaeaea1b015ae98b8e5ae28b845ae30000008c0446cc030cff003d343633393734373b49737261656c204f7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardReadout { card_number: 4639747, card_type: PunchCard, start: Some(SubSecond(SubSecondPunch { time: NaiveTime::from_str("18:27:46.556").unwrap(), day_of_week: Friday, week_counter: First })), finish: Some(SubSecond(SubSecondPunch { time: NaiveTime::from_str("18:27:47.517").unwrap(), day_of_week: Friday, week_counter: First })), check: Some(Punch { time: NaiveTime::from_str("18:27:53").unwrap(), day_of_week: Friday, week_counter: Second, code: 1 }), punches: vec![] };
    assert_eq!(
        CardReadout::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn empty_owner_data() {
    let mut data = hex::decode("7774cc91eaeaeaea1b015ae98b8e5ae28b845ae30000008c0446cc030cff003d3b3b3b3b3b3b3b3b3b3b3bee3b3beeee7269656e74656572696e67204173736f63696174696f6e3b3b3b3b3b3b48616b6661722048617961726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardOwnerData { first_name: "".to_string(), last_name: "".to_string(), gender: None, birthday: None, club: None, email: None, phone: None, city: None, street: None, zip: None, country: None };
    assert_eq!(
        CardOwnerData::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn normal_owner_data() {
    let mut data = hex::decode("7774cc91eaeaeaea1d0154dfeeeeeeeeeeeeeeee001f0eeb0446cc030cff003d746573743b746573743b746573743b746573743b746573743b746573743b746573743b746573743b746573743b746573743b746573743bee726f6b3b3b343738303030303b4953523bd1b4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f0025001f0170001f021d001f0221001f033f001f0341001f0343001f0377001f03eb0d1f59630d1f596e0d1f597d0d1f598b0d1f598feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
    let expected_data = CardOwnerData { first_name: "test".to_string(), last_name: "test".to_string(), gender: Some("test".to_string()), birthday: Some("test".to_string()), club: Some("test".to_string()), email: Some("test".to_string()), phone: Some("test".to_string()), city: Some("test".to_string()), street: Some("test".to_string()), zip: Some("test".to_string()), country: Some("test".to_string()) };
    assert_eq!(
        CardOwnerData::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}

#[tokio::test]
async fn full_owner_data() {
    let mut data = hex::decode("7774cc91eaeaeaea1d0154dfeeeeeeeeeeeeeeee001f0eeb0446cc030cff003d746573747465733b74657374746573743b746573743b74657374746573743b746573747465737474657374746573743b74657374746573747465737474657374746573743b74657374746573743b74657374746573743b74657374746573747465737474657374746573743b74657374746573743b74657374746573743beeee00000000000000000000000000000000001f0025001f0170001f021d001f0221001f033f001f0341001f0343001f0377001f03eb0d1f59630d1f596e0d1f597d0d1f598b0d1f598feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    #[rustfmt::skip]
        let expected_data = CardOwnerData { first_name: "testtes".to_string(), last_name: "testtest".to_string(), gender: Some("test".to_string()), birthday: Some("testtest".to_string()), club: Some("testtesttesttest".to_string()), email: Some("testtesttesttesttest".to_string()), phone: Some("testtest".to_string()), city: Some("testtest".to_string()), street: Some("testtesttesttesttest".to_string()), zip: Some("testtest".to_string()), country: Some("testtest".to_string()) };
    assert_eq!(
        CardOwnerData::from_card_blocks(&mut data, PunchCard)
            .await
            .unwrap(),
        expected_data
    );
}
