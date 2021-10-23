pub mod moanmyip {

    use std::net::{IpAddr, AddrParseError};
    use std::fs::File;
    use std::io::Cursor;
    use std::path::Path;
    use rand::seq::SliceRandom;
    use wav::BitDepth;

    pub struct Config {
        full_path: String,
        vector:  Vec<char>
    }

    impl Config {
        pub async fn new(address:String, path: &str) -> Config {
            let actors = vec!["development/"]; //, "zuzana/", "marie/", "katerina/", další dabing
            let voice = actors.choose(&mut rand::thread_rng()).unwrap();
            let full_path = format!("{}/{}", path, voice);

            let vector: Vec<char> = address.chars().map(|c| match c {
                '.' => 'x',
                ':' => 'z',
                _ => c
            }).collect();

            Config { full_path, vector}
        }
    }

    // Funkce pro kontrolu IP adresy
    pub async fn check_ip(ip: &str) -> Result<IpAddr, AddrParseError> {
        match ip.parse::<IpAddr>() {
            Ok(t) => Ok(t),
            Err(e) => Err(e)
        }
    }

    // Vytvoření WAV audia
    pub async fn create_moan(conf:Config) -> Result<Cursor<Vec<u8>>, std::io::Error> {
        let mut iter = conf.vector.iter();
        let first = iter.next().unwrap();

        // První část WAV souboru
        let (mut out, header) = get_first_vector(&conf.full_path, first).await?;

        for moan in iter {
            match get_wav_file(&conf.full_path, moan).await {
                Ok((data,_)) => {
                    // Přidání dalších znaků, bez hlavičky
                    out.append(&mut data.as_sixteen().unwrap().to_vec())
                },
                // Nenalezené písmena prostě přeskočíme
                Err(_) => {}
            }
        }

        let bitdepth = wav::BitDepth::from(out);
        let mut cursor = Cursor::new(Vec::new());
        // Zápis finálního WAVu do operační paměti
        wav::write(header, &bitdepth, &mut cursor)?;
        return Ok(cursor)
    }

    // Funkce pro získání první části audia
    // Obsahuje header WAV souboru
    async fn get_first_vector(path:&str, first:&char) -> Result<(Vec<i16>, wav::Header),std::io::Error> {
        match get_wav_file(&path, first).await {
            // S hlavičkou
            Ok((d,h)) => Ok((d.as_sixteen().unwrap().to_vec(),h)),
            Err(e)=> Err(e)
        }
    }

    // Funkce pro získání obsahu WAV jednotlivého souboru
    async fn get_wav_file(path:&str, c:&char) -> Result<(BitDepth, wav::Header),std::io::Error> {
        let filepath = format!("{}{}.wav", path, c);
        let mut file = File::open(Path::new(&filepath))?;
        let (header, data) = wav::read(&mut file)?;
        Ok((data, header))
    }

    // Funkce pro odstranění portu z IPv4 a IPv6 adresy
    pub async fn short_ip_address(ip:&str) -> &str {
        // Poslední dvojtečka, kompatiiblní s IPv6
        let position: usize = ip.rfind(':').unwrap();
        let address: &str = &ip[0..position];
        return address;
    }
}