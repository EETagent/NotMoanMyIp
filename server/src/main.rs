use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tide::Request;
use tide::{Body, Response, StatusCode};
use async_std::io::Cursor;
use maud::html;

use moanmyip_server::moanmyip;

// Hlavní funkce webové aplikace
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let host: String = match env::var("MOANMYIP_HOST") {
        Ok(ip) => ip,
        Err(_) => "0.0.0.0".to_string(),
    };
    let port: String = match env::var("MOANMYIP_PORT") {
        Ok(p) => p,
        Err(_) => "8080".to_string(),
    };

    let listener: String = format!("{}:{}", host, port);
    match listener.parse::<SocketAddr>() {
        Ok(_) => {}
        Err(e) => return Err(Box::new(e))
    }

    let mut app = tide::new();
    // Root
    app.at("/").get(homepage);
    // Podpůrné soubory
    app.at("/public").serve_dir("public")?;
    // Generované audio
    app.at("/output/:ip").get(api); app.at("/api/:ip").get(api);
    // Plaintext api pro IP adresu
    app.at("/simple").get(simple);
    // IP a port
    app.listen(listener).await?;
    Ok(())
}

// API pro generování audio souboru
async fn api(par: Request<()>) -> tide::Result {
    let address: &str = par.param("ip").unwrap();
    match moanmyip::check_ip(address).await {
        Ok(_) => (),
        Err(e) => return Ok(tide::Response::from(e.to_string()))
    };

    let conf = moanmyip::Config::new(address.to_string(), "wavmoans").await;

    let wav = match moanmyip::create_moan(conf).await {
        Ok(w) => Cursor::new(w.into_inner()),
        Err(e) => return Ok(tide::Response::from(e.to_string()))
    };

    let mut req = Response::new(StatusCode::Ok);
    req.set_body(Body::from_reader(wav, None));
    req.set_content_type("audio/wav");
    return Ok(req)
}

// Textové API pro IP adresu
async fn simple(par: Request<()>) -> tide::Result {
    let mut req = Response::new(StatusCode::Ok);
    req.set_content_type(tide::http::mime::PLAIN);
    let address = moanmyip::short_ip_address(par.remote().unwrap()).await;
    req.set_body(Body::from(address));
    return Ok(req)
}

// Hlavní stránka
async fn homepage(par: Request<()>) -> tide::Result {
    let mut req = Response::new(StatusCode::Ok);
    req.set_content_type(tide::http::mime::HTML);

    let address_full = par.remote().unwrap();
    let address = moanmyip::short_ip_address(address_full).await;
    let outfile = "/output/".to_owned() + &address;

    let web = html!{
        html{
            head {
                META HTTP-EQUIV="Pragma" CONTENT="no-cache";
                META HTTP-EQUIV="Expires" CONTENT="-1";
                
                link rel="stylesheet" href="/public/css/style.css";
            }
            body {
                img src="/public/img/moanmyip_lg.jpg" border="0";

                table class="buttons" border="0" {
                    tr{
                        td {
                            h4 {"Simple IP"}
                            a href="/simple" {
                                img src="/public/img/moanmyip_sm.gif" border="0";
                            }
                        }
                    }
                }
                div class="center" {
                    div class="content" {
                        h2{"Vaše veřejná IP adresa"}
                        div class="ip" {(address)}
                        center {
                            div id="audio-container" {
                            audio preload="auto" controls autoplay type="audio/wav" src=(outfile) {}
                            }
                        }

                    }
                }
                div class="subcontent" {
                    div class="subcontent_group" {
                        h4 {"Quick copy"}"Klikni na \"Zvýraznit\" níže a zmáčkni CTRL+C/CMD+C pro zkopírování tvé " b {"IP adresy"}
                        div id="copyforms" {
                            input type="button" value="Zvýraznit"  onclick="document.getElementById('myip').focus();";
                            input id="myip" type="text" value=(address_full) onfocus="this.select()" onclick="this.select()" readonly="readonly";
                        }
                    }
                }
            }
        }
    };
    req.set_body(Body::from(web.into_string()));
    return Ok(req);
}