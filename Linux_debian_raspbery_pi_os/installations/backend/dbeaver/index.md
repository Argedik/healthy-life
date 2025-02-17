flatpak --version
flatpak yükle
sudo apt update && sudo apt install flatpak -y
flathub deposunu yükle
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
dbeaver yükle
flatpak install flathub io.dbeaver.DBeaverCommunity -y

dbeaver başlat
flatpak run io.dbeaver.DBeaverCommunity
