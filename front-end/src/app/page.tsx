'use client';

import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { LineChart, Line, XAxis, YAxis, Tooltip } from 'recharts';

interface BesinVerisi {
  tarih: string;
  kalori: number;
}

const HomePage: React.FC = () => {
  const [veriler, setVeriler] = useState<BesinVerisi[]>([]);
  const [besinAdi, setBesinAdi] = useState('');
  const [miktar, setMiktar] = useState(0);

  useEffect(() => {
    fetchVeriler();
  }, []);

  const fetchVeriler = async () => {
    const response = await axios.get('/api/besin-verileri');
    setVeriler(response.data);
  };

  const besinEkle = async () => {
    await axios.post('/api/besin-ekle', {
      kullanici_id: 1, // Örnek kullanıcı ID
      besin_adi: besinAdi,
      miktar: miktar,
    });
    setBesinAdi('');
    setMiktar(0);
    fetchVeriler();
  };

  return (
    <div>
      <h1>Besin Değerleri Grafiği</h1>
      <div>
        <input
          type="text"
          placeholder="Besin Adı"
          value={besinAdi}
          onChange={(e) => setBesinAdi(e.target.value)}
        />
        <input
          type="number"
          placeholder="Miktar"
          value={miktar}
          onChange={(e) => setMiktar(parseFloat(e.target.value))}
        />
        <button onClick={besinEkle}>Besin Ekle</button>
      </div>
      <LineChart width={600} height={300} data={veriler}>
        <Line type="monotone" dataKey="kalori" stroke="#8884d8" />
        <XAxis dataKey="tarih" />
        <YAxis />
        <Tooltip />
      </LineChart>
    </div>
  );
};

export default HomePage;
