import { configureStore, combineReducers, getDefaultMiddleware } from '@reduxjs/toolkit'
import thunk from 'redux-thunk'
import groupListSlice from './Slices/groupListSlice'
import vkDataSlice from './Slices/vkDataSlice'
import activeSlice from './Slices/activeSlice'
import dateSLice from './Slices/dateSLice'
import groupNISlice from './Slices/groupNISlice'
import groupScheduleSlice from './Slices/groupScheduleSlice'
import accessTokenSlice from './Slices/accessTokenSlice'
import userDataSlice from './Slices/userDataSlice'
import fullNameEnabledSlice from './Slices/fullNameEnabledSlice'
import parsedScheduleSLice from './Slices/parsedScheduleSLice'
import planningDataSlice from './Slices/planningDataSlice'
import scheduleDiffsSlice from './Slices/scheduleDiffsSlice'

const rootReducer = combineReducers({
  // stateName: sliceName,
  groupList: groupListSlice,
  vkData: vkDataSlice,
  active: activeSlice,
  date: dateSLice,
  groupNI: groupNISlice,
  groupSchedule: groupScheduleSlice,
  accessToken: accessTokenSlice,
  userData: userDataSlice,
  fullNameEnabled: fullNameEnabledSlice,
  parsedSchedule: parsedScheduleSLice,
  planningData: planningDataSlice,
  scheduleDiffs: scheduleDiffsSlice,
});

export default configureStore({
  reducer: rootReducer,
  middleware: (getDefaultMiddleware) => (
    getDefaultMiddleware().concat(thunk)
  )
});
