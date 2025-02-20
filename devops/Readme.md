sunucunun http (80) portunu terminalden başlatmak için 
& karakteri sunucunun arka planda calısmasını sağlıyor
sudo target/release/devops &



# tekrar derleyip calıstırmak için
cargo build --release
sudo target/release/devops &
# sunucu kontrolü
sudo netstat -tulnp | grep -E '80|443'

# Mevcutta çalışan Rust sunucularını kapatır.
sudo pkill -f target/release/devops
