import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { LineChart, Line, XAxis, YAxis, Tooltip } from 'recharts';

interface BesinVerisi {
	tarih: string;
	kalori: number;
}

const HomePage: React.FC = () => {
	const [veriler, setVeriler] = useState<BesinVerisi[]>([]);

	useEffect(() => {
		axios.get('http://localhost:8080/besin-verileri').then((response) => {
			setVeriler(response.data);
		});
	}, []);

	return (
		<div>
			<h1>Besin Değerleri Grafiği</h1>
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
