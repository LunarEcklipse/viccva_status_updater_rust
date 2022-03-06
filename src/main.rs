use reqwest;
use reqwest::header::{AUTHORIZATION, HeaderName};
use std::time::{Duration, SystemTime};
use serde::Deserialize;
use std::thread;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const CLIENT_ID: &str = "[CLIENT-ID-HERE]";
const CLIENT_SECRET: &str = "[CLIENT-SECRET-HERE]";

static mut AUTH_REFRESH: bool = true;


#[derive(Deserialize)]
struct TwitchAuthRaw
{
    access_token: String,
    expires_in: i64,
    token_type: String,
}

struct TwitchAuth
{
    access_token: String,
    creation_time: SystemTime,
    expiration_seconds: i64,
    data_valid: bool,
}

async fn refresh_twitch_data(client: &reqwest::Client, auth: &TwitchAuth)
{
    let auth_start = String::from("Bearer ");
    let auth_text = format!("{}{}", auth_start, auth.access_token);
    let client_id = String::from(CLIENT_ID);
    drop(auth_start);
    {
        let channel_url = "https://api.twitch.tv/helix/channels?broadcaster_id=518336885";
        let res = client.get(channel_url)
            .header(AUTHORIZATION, &auth_text)
            .header(HeaderName::from_static("client-id"), &client_id)
            .send()
            .await
            .unwrap();
        match res.status()
        {
            reqwest::StatusCode::OK =>
            {
                let out = res.text().await.unwrap();
                let path = Path::new("./out/channel_data.json");
                let display = path.display();
                let mut f = match File::create(&path)
                {
                    Err(why) => panic!("Couldn't write to file at {}: {}", display, why),
                    Ok(f) => f,
                };
                match f.write_all(out.as_bytes())
                {
                    Err(why) => panic!("Couldn't write to file at {}: {}", display, why),
                    Ok(_) => (),
                };
            },
            reqwest::StatusCode::BAD_REQUEST =>
            {
                let outstr = res.text().await.unwrap();
                println!("Server returned Bad Request. Error reads: {}", outstr);
                return;
            },
            reqwest::StatusCode::UNAUTHORIZED =>
            {

                println!("Server returned Unauthorized. Key is likely expired; setting key to refresh. Error reads {}", res.text().await.unwrap());
                unsafe
                {
                    AUTH_REFRESH = true;
                };
                return;
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR =>
            {
                println!("Server returned Internal Server Error. Trying again in 5 seconds...");
                return;
            },
            _ =>
            {
                panic!("ERROR: Server returned an unexpected response code. Response code was: {:?}", res.status());
            }
        };
    }
    {
        let stream_url = "https://api.twitch.tv/helix/streams?user_id=518336885";
        let res = client.get(stream_url)
            .header(AUTHORIZATION, &auth_text)
            .header(HeaderName::from_static("client-id"), &client_id)
            .send()
            .await
            .unwrap();
            match res.status()
            {
                reqwest::StatusCode::OK =>
                {
                    let out = res.text().await.unwrap();
                    let path = Path::new("./out/stream_data.json");
                    let display = path.display();
                    let mut f = match File::create(&path)
                    {
                        Err(why) => panic!("Couldn't write to file at {}: {}", display, why),
                        Ok(f) => f,
                    };
                    match f.write_all(out.as_bytes())
                    {
                        Err(why) => panic!("Couldn't write to file at {}: {}", display, why),
                        Ok(_) => (),
                    };
                },
                reqwest::StatusCode::BAD_REQUEST =>
                {
                    let outstr = res.text().await.unwrap();
                    println!("Server returned Bad Request. Error reads: {}", outstr);
                    return;
                },
                reqwest::StatusCode::UNAUTHORIZED =>
                {
    
                    println!("Server returned Unauthorized. Key is likely expired; setting key to refresh. Error reads {}", res.text().await.unwrap());
                    unsafe
                    {
                        AUTH_REFRESH = true;
                    };
                    return;
                }
                reqwest::StatusCode::INTERNAL_SERVER_ERROR =>
                {
                    println!("Server returned Internal Server Error. Trying again in 5 seconds...");
                    return;
                },
                _ =>
                {
                    panic!("ERROR: Server returned an unexpected response code. Response code was: {:?}", res.status());
                }
            };
    }
    {
        let user_url = "https://api.twitch.tv/helix/users?id=518336885";
        let res = client.get(user_url)
            .header(AUTHORIZATION, &auth_text)
            .header(HeaderName::from_static("client-id"), &client_id)
            .send()
            .await
            .unwrap();
            match res.status()
            {
                reqwest::StatusCode::OK =>
                {
                    let out = res.text().await.unwrap();
                    let path = Path::new("./out/user_data.json");
                    let display = path.display();
                    let mut f = match File::create(&path)
                    {
                        Err(why) => panic!("Couldn't write to file at {}: {}", display, why),
                        Ok(f) => f,
                    };
                    match f.write_all(out.as_bytes())
                    {
                        Err(why) => panic!("Couldn't write to file at {}: {}", display, why),
                        Ok(_) => (),
                    };
                },
                reqwest::StatusCode::BAD_REQUEST =>
                {
                    let outstr = res.text().await.unwrap();
                    println!("Server returned Bad Request. Error reads: {}", outstr);
                    return;
                },
                reqwest::StatusCode::UNAUTHORIZED =>
                {
    
                    println!("Server returned Unauthorized. Key is likely expired; setting key to refresh. Error reads {}", res.text().await.unwrap());
                    unsafe
                    {
                        AUTH_REFRESH = true;
                    };
                    return;
                }
                reqwest::StatusCode::INTERNAL_SERVER_ERROR =>
                {
                    println!("Server returned Internal Server Error. Trying again in 5 seconds...");
                    return;
                },
                _ =>
                {
                    panic!("ERROR: Server returned an unexpected response code. Response code was: {:?}", res.status());
                }
            };
    }
}

async fn get_twitch_auth(client: &reqwest::Client) -> TwitchAuth
{
    println!("Refreshing Twitch authentication key...");
    let params = [("client_id", CLIENT_ID), ("client_secret", CLIENT_SECRET), ("grant_type", "client_credentials")];
    let res = client.post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await
        .unwrap();
    drop(params);
    match res.status()
    {
        reqwest::StatusCode::OK =>
        {
            match res.json::<TwitchAuthRaw>().await
            {
                Ok(parsed) =>
                {
                    println!("Authentication token successfully acquired.");
                    let ret = TwitchAuth
                    {
                        access_token: parsed.access_token,
                        creation_time: SystemTime::now(),
                        expiration_seconds: parsed.expires_in,
                        data_valid: true,
                    };
                    unsafe
                    {
                        AUTH_REFRESH = false;
                    };
                    return ret;
                }
                Err(_) =>
                {
                    println!("JSON Parse failed. Check for a breaking API change.");
                    let ret = TwitchAuth
                    {
                        access_token: String::from(""),
                        creation_time: SystemTime::now(),
                        expiration_seconds: 0,
                        data_valid: false,
                    };
                    return ret;
                }
            }
        },
        reqwest::StatusCode::BAD_REQUEST =>
        {
            println!("Server returned Bad Request. Check your client ID and secret, or look up for a breaking API change.");
            let ret = TwitchAuth
            {
                access_token: String::from(""),
                creation_time: SystemTime::now(),
                expiration_seconds: 0,
                data_valid: false,
            };
            return ret;
        },
        reqwest::StatusCode::INTERNAL_SERVER_ERROR =>
        {
            println!("Server returned Internal Server Error. Trying again in a moment.");
            let ret = TwitchAuth
            {
                access_token: String::from(""),
                creation_time: SystemTime::now(),
                expiration_seconds: 0,
                data_valid: false,
            };
            return ret;
        },
        _ =>
        {
            panic!("ERROR: Server returned an unexpected response code. Response code was: {:?}", res.status());
        }
    }
}

fn check_auth_validity(auth: &TwitchAuth) -> bool
{
    if !(auth.data_valid)
    {
        return false;
    }
    let timenow = SystemTime::now();
    let diff = timenow.duration_since(auth.creation_time);
    let diff = match diff
    {
        Ok(difference) => difference,
        Err(_error) => // If the time doesn't make sense, we just return false.
        {
            return false;
        }
    };
    let max_duration = time::Duration::new(auth.expiration_seconds - 3600, 0);
    if diff > max_duration // Reauthenticate if there's less than an hour left
    {
        return false;
    }
    return true;
}

#[tokio::main]
async fn main()
{
    let http_cl = reqwest::Client::new();
    let mut auth = get_twitch_auth(&http_cl).await;
    match fs::create_dir("out")
    {
        Ok(_) => (),
        Err(_) => (),
    }
    loop
    {
        let auth_check;
        unsafe
        {
            auth_check = AUTH_REFRESH;
        };
        if !(check_auth_validity(&auth)) || auth_check
        {
            auth = get_twitch_auth(&http_cl).await;
        }
        else
        {
            refresh_twitch_data(&http_cl, &auth).await;
        }
        thread::sleep(Duration::from_secs(5));
    }
}
