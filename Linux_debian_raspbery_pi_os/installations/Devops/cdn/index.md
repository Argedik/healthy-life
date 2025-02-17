Content Delivery Network
Sunucu trafiği diye anladım. 
Müşteri web sitesini açtığında görüntülenen html css js içeriklerinin hepsini hangi sunucudan çekmesi gerektiğini yönetiyoruz.
Almanyadaki birinin türkiyedeki web sitesini açmak için bir kaç katı kadar yavaş açılan bir site olmaması adına CDN yöntemi kullanılıyor.
Bunun için birden fazla fiziksel sunucuyu bölgelere dağıtmamız gerekiyor. Her kullanıcı hangi sunucuya yakında oradan tüm site içeriklerini yüklemesi gerekiyor.
Müşterimiz bir video yüklediğinde tüm sunuculara dağıtmak yerine merkez sunucuya yükleyip daha sonra videoyu çağırma aşamasında 1 defaya mahsus müşteriye yakın olan sunucuya da yüklenir bu süre ilk video çağırmada uzun sürebilir ama daha az maliyet için gerekli olabilir. Daha hızlı çözüm olarak tüm sunuculara yüklenebilir.