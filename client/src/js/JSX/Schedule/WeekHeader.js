import { useSelector } from "react-redux";
import { weekHeaderTime, isEvenWeek } from "../../Utils/handleTime";
import { useState, useEffect } from "react";

export function WeekHeader({weekParity}) {
  const {active} = useSelector(s => s.active);
  const {date, weekNumber} = useSelector(s => s.date);
  const {groupNumber, groupId} = useSelector(s => s.groupNI);

  const [clock, setClock] = useState(weekHeaderTime(new Date()));

  useEffect(() => {
    const intervalId = setInterval(() => {
      setClock(weekHeaderTime(new Date()));
    }, 1000);

    return () => {
      clearInterval(intervalId);
    };
  }, []);
  

  return (
    <>
    <div className='schedule__info schedule-info'>
      <div className='schedule-info__group schedule-info__item'>Группа: {groupNumber}</div>
      {active === 'schedule' && <div className='schedule-info__date schedule-info__item'>Дата: {clock[0]}. Время: {clock[1]}</div>}
      <div className='schedule-info__week-parity schedule-info__item'>
        {weekParity ? 
          `Неделя:  ${weekParity}`
          :
          `Неделя:  ${isEvenWeek(date)}`
        }
      </div>
    </div>
    </>
  )
}