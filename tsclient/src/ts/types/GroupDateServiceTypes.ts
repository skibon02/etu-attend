export interface IGroupDateService {
  groupIdSetFetch(groupId: number, groupNumber: string): void,
  groupNumberIdGetFetch(): void,
  groupScheduleGetFetch(groupId: number): void,
  scheduleDiffsSETFetch(time_link_id: number, weekNumber: number, flag: boolean): void,
  schedulePlanningSETOneFetch(time_link_id: number, flag: boolean): void,
  schedulePlanningSETAllFetch(flag: boolean): void,
}