// interface BesinVerisi {
// 	tarih: string;
// 	kalori: number;
// }
import style from './style.module.scss';

const Example: React.FC = () => {
	return (
		<div className={`${style.container}`}>
			<div id="item-1" className={`${style.item1}`}>
				item - 1
			</div>
			<div id="item-2">item - 2</div>
			<div id="item-3">item - 3</div>
			<div id="item-4">item - 4</div>
		</div>
	);
};

export default Example;
