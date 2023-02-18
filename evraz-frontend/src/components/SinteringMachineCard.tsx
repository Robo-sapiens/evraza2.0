/*
Информация по каждому эксгаустеру отображается под названием агломашины к которой эксгаустер принадлежит и содержит:
- Поле «Стасту работы» состоит из: 
    - <зеленый индикатор> - эксгаустер в работе
    - <красный индикатор> - эксгаустер не в работе
    - кнопка для перехода в экран «Эксгаустер У – 171 … Х - 172»
- Поле «Номер ротора» состоит из:
    - Ротор №<номер ротора> - номер ротора соответствует записи, внесенной в справочник роторов
    - <DD.MM.YYYY> - указывает дату замены ротора на конкретном эксгаустере;
    - <кнопка изменить> - НЕ ИСПОЛЬЗУЕТСЯ
- Поле «Последняя замена ротора» состоит из:
    - "<unsigned> сут" - отображается суммарно время, которое установленный ротор был в работе;
    - "<unsigned> сут" - отображается прогнозное время работы до отказа, дополнительная индикация в виде значков:
        - <красный кружок> - предупреждение о времени работы от 3 дней и менее;
        - <желтый кружок> - предупреждение о времени работы от 10 дней и менее;
- Поле «Предупреждений» состоит из:
    - <схема> - схематичное изображение эксгаустера, на котором подсвечиваются подшипники при наведении курсора мыши на соответствующие точки измерения в таблице «Подшипники» или «Предупреждения»;
    - таблица «Предупреждения» отображает строки с точками измерений, которые имеют статус «Предупреждение» или «Авария»   (точка измерения, отображаемая в таблице «Предупреждения», убирается из списка в таблице «Подшипники»);
    - таблица «Подшипники» отображает строки с точками измерений, которые не имеют статуса «Предупреждение» или «Авария» (при возникновении у точки измерения статуса «Предупреждение» или «Авария», запись перемещается в таблицу «Предупреждения»).
*/

import ExhausterItem, { ExhausterProps } from "./ExhausterItem";

// По описанию выше необходимо составить компонент, который будет отображать информацию по каждому эксгаустеру отображается под названием агломашины к которой эксгаустер принадлежит и содержит:
// Пропсы компонента SinteringMachineCard:
export interface SinteringMachineCardProps {
    // Название агломашины
    name: string;

    exhausters: ExhausterProps[];
}

const SinteringMachineCard = (props: SinteringMachineCardProps) => {
    return (
      <div className="bg-white">
          <div className="bg-stone-100 p-2 rounded">
              {props.name}
          </div>
          <div className="flex gap-2 pt-2 items-start">
                {props.exhausters.map((exhauster, index) => (
                    <ExhausterItem key={index} {...exhauster} />
                ))}
          </div>
      </div>
    )
}


export default SinteringMachineCard