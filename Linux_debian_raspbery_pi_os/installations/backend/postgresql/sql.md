Örnek tablo oluşturarak başlıyoruz

`
CREATE TABLE categories (
    number SERIAL PRIMARY KEY,
    png_url TEXT NOT NULL,
    description TEXT NOT NULL
);
`
tablonun kolon ve kolonların tiplerini öğrenme 
SELECT column_name, data_type
FROM information_schema.columns
WHERE table_name = 'categories';


//verileri yüklüyoruz
INSERT INTO categories (png_url, description) VALUES
('https://example.com/salata.png', 'Salatalar'),
('https://example.com/icecek.png', 'Lezzetli içecekler'),
('https://example.com/sebze.png', 'Sebze yemekleri'),
('https://example.com/ornek.png', 'Örnek Listeler'),
('https://example.com/gece.png', 'Gece Kaçamakları'),
('https://example.com/istatistik.png', 'Ne alıyorum istatistik');