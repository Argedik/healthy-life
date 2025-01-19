Kaynak: https://chatgpt.com/c/678b55fb-e9d8-800d-8686-79a3f90c34a7
Ubuntu Linux'ta Nginx Nasıl Kurulur ve Test Edilir?

Bu döküman, Ubuntu Linux sisteminizde Nginx web sunucusunun nasıl kurulacağını ve kurulum sonrası test işlemlerinin nasıl yapılacağını açıklamaktadır.

1. Depoları Güncelleyin

İlk olarak, sistemdeki paketlerin güncel olduğundan emin olun:

sudo apt update

2. Nginx'i Yükleyin

Ubuntu depolarından Nginx'i yüklemek için şu komutu çalıştırın:

sudo apt install nginx -y

Bu komut, Nginx'in son kararlı sürümünü sisteminize kurar.

3. Nginx Hizmetini Başlatın

Kurulumdan sonra Nginx hizmetini başlatın:

sudo systemctl start nginx

Nginx'in her sistem başlatıldığında otomatik olarak çalışması için:

sudo systemctl enable nginx

4. Nginx'in Çalışıp Çalışmadığını Kontrol Edin

Hizmetin durumunu kontrol etmek için şu komutu kullanabilirsiniz:

sudo systemctl status nginx

Eğer active (running) görüyorsanız, Nginx başarılı bir şekilde çalışıyor demektir.

5. Varsayılan Nginx Web Sayfasını Test Edin

Tarayıcınızı açın ve şu adresi ziyaret edin:

http://localhost

Eğer "Welcome to Nginx!" mesajını içeren varsayılan Nginx sayfasını görüyorsanız, kurulum başarılıdır.

---

6. Firewall Ayarlarını Kontrol Edin (Opsiyonel)

Eğer tarayıcıdan erişimde sorun yaşıyorsanız, Nginx için gerekli portların açık olduğundan emin olun:

UFW'nin durumunu kontrol edin:

sudo ufw status

Eğer UFW etkinse, HTTP ve HTTPS trafiğine izin verin:

sudo ufw allow 'Nginx Full'

Değişikliklerden sonra UFW'yi yeniden yükleyin:

sudo ufw reload

7. Nginx Loglarını Kontrol Edin

Nginx'in düzgün çalışıp çalışmadığını anlamak için log dosyalarını inceleyebilirsiniz:

Erişim logları:

sudo tail -f /var/log/nginx/access.log

Hata logları:

sudo tail -f /var/log/nginx/error.log

8. Nginx Portunu Kontrol Edin

Nginx'in hangi portları kullandığını görmek için şu komutu çalıştırabilirsiniz:

sudo netstat -tuln | grep :80

Eğer 80 ve/veya 443 portları açık görünüyorsa, Nginx başarılı bir şekilde çalışıyordur.

9. Nginx'in Yapılandırmasını Test Edin

Nginx yapılandırma dosyalarını test etmek için:

sudo nginx -t

Eğer "syntax is ok" ve "test is successful" mesajlarını görüyorsanız, yapılandırma hatasızdır.

10. Nginx Hizmetini Yeniden Başlatma

Yapılandırma dosyalarını değiştirdikten sonra, Nginx hizmetini yeniden başlatmanız gerekebilir:

sudo systemctl restart nginx

Bu adımları takip ederek Ubuntu sisteminizde Nginx'i kurabilir ve test edebilirsiniz. Daha fazla yardıma ihtiyacınız olursa, lütfen belirtin!