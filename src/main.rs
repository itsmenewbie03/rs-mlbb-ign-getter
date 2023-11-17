use reqwest;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
struct PaymentConfirmation {
    initCallBackendAPI: bool,
    orderId: String,
    errorCode: String,
    confirmation: bool,
    isUserConfirmation: bool,
    errorMsg: String,
    paymentChannel: String,
    result: String,
    channelPrice: String,
    confirmationFields: ConfirmationFields,
    success: bool,
    denom: String,
    user: User,
    isThirdPartyMerchant: bool,
    txnId: String,
}

#[derive(Debug, Deserialize)]
struct ConfirmationFields {
    zipCode: String,
    country: String,
    totalPrice: String,
    create_role_country: String,
    userIdAndZoneId: String,
    userId: String,
    productName: String,
    paymentChannel: String,
    this_login_country: String,
    channelPrice: String,
    zoneId: String,
    verifiedMsisdn: String,
    taxAmount: String,
    email: String,
    inputRoleId: String,
    username: String,
}

#[derive(Debug, Deserialize)]
struct User {
    userId: String,
    zoneId: String,
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Specify the URL you want to make a GET request to
    let url = "https://order-sg.codashop.com/initPayment.action";
    let form_data = "voucherPricePoint.id=266837&voucherPricePoint.price=5.5&voucherPricePoint.variablePrice=0&n=11%2F17%2F2023-2121&email=&userVariablePrice=0&order.data.profile=eyJuYW1lIjoiICIsImRhdGVvZmJpcnRoIjoiIiwiaWRfbm8iOiIifQ%3D%3D&user.userId=619696143&user.zoneId=10115&msisdn=&voucherTypeName=MOBILE_LEGENDS&voucherTypeId=5&gvtId=19&lvtId=51&pcId=90&shopLang=en_PH&checkoutId=f31cc7ce-9973-4b59-94f1-4197c6b76819&affiliateTrackingId=&impactClickId=&anonymousId=&absoluteUrl=https%3A%2F%2Fwww.codashop.com%2Fen-ph%2Fmobile-legends&utmParameters=&userSessionId=f23dce35-f4ac-49f7-9611-6bd71fe445d5&userEmailConsent=false&userMobileConsent=false&userCustomCommerceEmailConsent=false&verifiedMsisdn=&promoId=&promoCode=&clevertapId=&promotionReferralCode=&isReferredUser=false";
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(form_data.to_owned())
        .send()
        .await?;
    if response.status().is_success() {
        // Print the response body as a string
        let body = response.text().await?;
        let parsed_body: Result<PaymentConfirmation, serde_json::Error> =
            serde_json::from_str(body.as_str());
        match parsed_body {
            Ok(data) => {
                println!(
                    "IGN: {:?}\nGrabbed using Rust🦀",
                    data.confirmationFields.username
                );
            }
            Err(e) => {
                // Handle the error if parsing fails
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    } else {
        // Print an error message if the request was not successful
        println!("Request failed with status code: {}", response.status());
    }
    Ok(())
}
