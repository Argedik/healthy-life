
### Docker'ın çalışıp çalışmadığı kontrolü
- systemctl status docker
### Sistemdeki kurulu paketler arasında “docker” ismi geçenleri listeleyen komut.
- dpkg -l | grep docker

### Çalışan containerları kontrol et
- sudo docker ps

### Çalışmayan ve oluşturulmuş olan tüm container ları gör
- sudo docker ps -a
- docker images
