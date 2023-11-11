import myfetch from "../myfetch";

async function getGroupList(setGroupList, setGroupListError) {
  try {
    let response = await myfetch('/api/groups');
    if (!response.ok) {
      throw new Error(`Failed to myfetch: ${response.status} ${response.statusText}`);
    }
    let data = await response.json();

    let groups = [];
    for (let k of Object.keys(data)) {
      groups.push({
        ...data[k],
        id: k
      });
    }
    console.log('groups:');
    console.log(groups);

    setGroupList(groups);
  } catch (error) {
    setGroupListError(error.message);
  }
}

async function getVkData(setVkData) {
  let response = await myfetch('/api/auth/data');
  let data = await response.json();
  setVkData(data);
  console.log('vk data:');
  console.log(data);
}


function getGroupSchedule(groupId, setGroupSchedule) {
  if (groupId) {
    async function getSchedule() {
      // setGroupSchedule(null);
      let response = await myfetch(`/api/scheduleObjs/group/${groupId}`);
      let data = await response.json();
      console.log('successful fetch on Schedule\nresponse.json():\n', data);
      // console.log('response.body:');
      // console.log(response.body);
      
      setGroupSchedule(data);
      // localStorage.setItem('userPref', data);
      // alert(localStorage.getItem('userPref'))
    }

    getSchedule();
  }
}


export {
  getGroupList,
  getGroupSchedule,
  getVkData,
}
