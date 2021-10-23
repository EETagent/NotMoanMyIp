use std::io::{BufReader, Cursor, Write};
use std::fs::File;
use std::net::IpAddr;
use clap::{Arg, App};
use url::{ParseError, Url};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let application = App::new("Moan webservice client")
        .version("1.1")
        .about("Terminal client for Moan webservice")
        .arg(Arg::new("ip")
            .about("Sets a IP")
            .required(true)
            .index(1))
        .arg(Arg::new("server")
            .short('s')
            .long("server")
            .about("Moan webservice server")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("download")
            .short('d')
            .long("download")
            .about("Downloads WAV audio")
            .takes_value(false))
        .get_matches();

    // Obě dvě hodnoty jsou .required(), unwrap je bezpečný
    let ip: &str = application.value_of("ip").unwrap();
    let server: &str = application.value_of("server").unwrap();

    // Vyhodnocení IP adresy
    match ip.parse::<IpAddr>() {
        Ok(_) => {
            // Převod vstupu na {URL}/api
            match parse_moanmyip_url(server, ip).await {
                Ok(url) => {
                    // Stažení audio souboru
                    match surf::get(url.as_str()).recv_bytes().await {
                        Ok(v) => {
                            if !v.is_empty() {
                                // Uložit do souboru jestli je aktivní přepínač download
                                if application.is_present("download") {
                                    let filepath = ip.to_string() + ".wav";
                                    match File::create(filepath) {
                                        Ok(mut f) => {
                                            f.write_all(&v).unwrap();
                                        }
                                        Err(e) => eprintln!("{}", e)
                                    };
                                // Jinak přehrát na zařízení
                                } else {
                                    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                                    let sink = rodio::Sink::try_new(&handle).unwrap();
                                    let cursor = Cursor::new(v);
                                    sink.append(rodio::Decoder::new(BufReader::new(cursor)).unwrap());
                                    sink.sleep_until_end();
                                }
                            }
                        }
                        // Nepodařilo se stažení dat
                        Err(e) => eprintln!("{}", e)
                    }
                }
                // Neplatné URL
                Err(e) => eprintln!("{}", e)
            }
        }
        // Neplatná IP adresa
        Err(e) => eprintln!("{}", e)
    }
    Ok(())
}

// Zpracování adresy
async fn parse_moanmyip_url(server: &str, ip: &str) -> Result<Url, ParseError> {
    let mut base_url = Url::parse(server)?;
    base_url.set_path(&*format!("api/{}", ip));
    Ok(base_url)
}
