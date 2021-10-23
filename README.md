<h1 class="rich-diff-level-zero">Not MoanMyIP - What's my IP co má šmrnc</h1>

Velmi jednoduchá implementace internetové služby moanmyip.com, která narozdíl od originálního projektu poskytuje snadnou API

![Ukázka stránky](.github/img/moanmyip.png?raw=true)

## Server

Frontend a backend projektu. Velmi svižný a minimalistický. Skládá IP adresu z hlásek, které ma k dispozici. Vše probíhá v paměti RAM, soubory se nezapisují na disk ale rovnou servírují klientovi. Úložný prostor serveru na úkor škálovatelnosti. Kešování atd. nebude implementováné, celý projekt je tak trochu na rychlo vytvořená recese xd

## Client

CLI program, který slouží jako klient k serverové službě. Přehrává i stahuje audio

![Ukázka stránky](.github/img/client.png?raw=true)


## Kompilace

K sestavení projektu jsou potřeba následující závislosti:

* Rust

Stažení projektu:

```bash
git clone https://github.com/EETagent/NotMoanMyIp.git --depth 1
```

Sestavení aplikace:  

```bash
cargo build --release
```