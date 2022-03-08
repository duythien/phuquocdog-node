use frame_support::{assert_ok, assert_noop};
use crate::{self as pallet_currency, mock::*, Error};
#[test]
fn add_currency_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Currencies::add_currency(
			Origin::signed(1),
			String::from("PQD").into_bytes(),
			10,
			String::from("wss://rpc.phuquoc.dog").into_bytes(),
			true
		));

		assert_eq!(
			Currencies::currency(String::from("PQD").into_bytes()),
			Some(pallet_currency::CurrencyInfo {
				decimal: 10,
				rpc_url: String::from("wss://rpc.phuquoc.dog").into_bytes(),
				native: true
			})
		);
	})
}

#[test]
fn cant_transfer_when_currency_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Currencies::transfer(Origin::signed(1), 2, String::from("AUSD").into_bytes(), 21000000),
			Error::<Test>::CurrencyNotExist
		);
	})
}