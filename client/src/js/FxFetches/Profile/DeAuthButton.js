import myfetch from "../myfetch";

async function deauth(setVkData) {
  await myfetch('/api/auth/deauth', {method: "POST"} )
  setVkData({})
  localStorage.clear();
}

export {
  deauth,
}