cmake ve clang kütüphanelerini yüklemek gerekiyor aws-lc-sys erroru almamak için

client_secret.apps.googleusercontent.com.json dosyasındaki bilgiler tanımlanmalı

urller canlıya alındığında tekrardan kontrol edilmeli

Middleware, Routing, Controller, Service, Database, Thirt-Party API, Http (İstemci -> sunucu), WebSocket (Sunucu <-> İslemci)


***Testler***
1- cargo run yazdığımızda terminalde "Sunucu çalışıyor: http://127.0.0.1:8080" görülmeli

2- http://127.0.0.1:8080/fridge_items 
adresine Postman’de “Body” sekmesini JSON olarak ayarla ve aşağıdaki içeriği koy.
{
  "image_url": "http://example.com/apple.jpg",
  "title": "Apple"
}

true dönmeli

3- Postman’de sadece GET seçip http://127.0.0.1:8080/fridge_items
Dönüş örneği;
[
  {
    "id": 1,
    "image_url": "http://example.com/apple.jpg",
    "title": "Apple"
  }
]

4- Postman put / URL: http://127.0.0.1:8080/fridge_items/1 (1 → güncellenecek satırın id değeri)
Body kısmına;
{
  "id": null,
  "image_url": "http://example.com/red_apple.jpg",
  "title": "Red Apple"
}

true dönmeli

5- Postman’de DELETE metodu ile http://127.0.0.1:8080/fridge_items/1.
istek sonucu true dönmeli ve GET /fridge_items yaptığımızda [] dönmeli

//veri ekleme
Invoke-WebRequest -Uri "http://127.0.0.1:3000/fridge_items" `
  -Method POST `
  -ContentType "application/json" `                                                                                  
  -Body '{"image_url": "http://   example.com/apple.jpg", "title": "Apple"}'   