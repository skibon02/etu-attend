import CALENDAR from './../../icons/calendar-pen.svg'
import NAVCLOCK from './../../icons/clock-for-nav.svg'
import SEARCH from '../../icons/search-for-nav.svg'

export default function Header({date, setDate, active, setActive, setGroupSchedule, setGroup}) {

  function handleNextWeek() {
    setDate(new Date(date.getTime() + 24 * 60 * 60 * 1000 * 7));
  }

  function handleCurrentWeek() {
    setDate(new Date());
  }

  function handlePrevWeek() {
    setDate(new Date(date.getTime() - 24 * 60 * 60 * 1000 * 7));
  }

  function handleScheduleClick() {
    setActive('schedule');
  }

  function handlePlanningClick() {
    setActive('planning');
  }

  function handleGroupsClick() {
    setActive('groups');
    setGroupSchedule(null);
    setGroup(null);
  }

  return (
    <div className="header">
      <div className="header__nav nav">
        <div 
          className="nav__item header-hover"
          onClick={handleScheduleClick} >
          <div className='nav__icon-container'>
            <img className='nav__icon nav__shitty-clock' src={NAVCLOCK} alt="calendar" />
          </div>
          <span className='nav__text'>Расписание</span>
        </div>
        <div 
          className="nav__item header-hover"
          onClick={handlePlanningClick} >
          <div className='nav__icon-container'>
            <img className='nav__icon' src={CALENDAR} alt="calendar" />
          </div>
          <span className='nav__text'>Планирование</span>
        </div>
        <div 
          className="nav__item header-hover"
          onClick={handleGroupsClick} >
          <div className='nav__icon-container'>
            <img className='nav__icon' src={SEARCH} alt="calendar" />
          </div>
          <span className='nav__text'>Группы</span>
        </div>
      </div>
      {active === 'schedule' && <div className="header__week-buttons">
        <div 
          className='header__week-button header-hover' onClick={handlePrevWeek}>
            К предыдущей неделе
        </div> 
        <div 
          className='header__week-button header-hover' onClick={handleCurrentWeek}>
          К текущей неделе
        </div> 
        <div 
          className='header__week-button header-hover' onClick={handleNextWeek}>
            К следующей неделе
        </div> 
        {/* <div className="header__time header-time">
          <span className='header-time__text'>Дата и время:</span>
          <br />
          {clock}
        </div> */}
      </div>}
    </div>
  )
}