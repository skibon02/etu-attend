// import { useState, useEffect } from 'react';
// import { Config } from '@vkontakte/superappkit';
// import myfetch from '../../FxFetches/myfetch';
// import { Connect, ConnectEvents } from '@vkontakte/superappkit';
// import { isdev, currentHost } from '../../FxFetches/util';
// import { useDispatch } from 'react-redux';
// import { vkDataFetch } from '../../ReduxStates/Slices/vkDataSlice';

// // vk id штучка
// Config.init({
//   appId: 51771477, // идентификатор приложения
// });

// const SERVER_HOST = currentHost;

// export default function VkButton() {
//   const dispatch = useDispatch();

//   const [authData, setAuthData] = useState(null);
//   const [isLoading, setIsLoading] = useState(false);

//   useEffect(() => {
//     dispatch(vkDataFetch());
//   }, [isLoading]);

//   useEffect(() => {
//     async function Fetches() {
//       console.log('start LOADING AUTHORIZE');
//       await VladFetch();
//       // setTimeout(() => {
//       //     getVkData(setVkData);
//       //     setIsLoading(false);
//       // }, 3000);
//       console.log('LOADING AUTHORIZE finished');
//       setIsLoading(!isLoading);
//     }

//     Fetches();
//   }, [authData])

//   async function VladFetch() {
//     const vkAuthRedirectURL = '/api/authorize';
//     if (authData) {
//       await myfetch(vkAuthRedirectURL, {
//         credentials: 'include',
//         method: 'POST',
//         headers: {
//           'Content-Type': 'application/json;charset=utf-8'
//         },
//         body: JSON.stringify({
//           silent_token: authData.token,
//           uuid: authData.uuid,
//         })
//       });
//     }
//   }

//   useEffect(() => {
//     const vkAuthRedirectURL = SERVER_HOST+'/api/auth/redirect';
//     const vkOneTapButton = Connect.buttonOneTapAuth({
//         // Обязательный параметр в который нужно добавить обработчик событий приходящих из SDK
//       callback: function(e) {
//         const type = e.type;

//         if (!type) {
//           return false;
//         }

//         switch (type) {
//           case ConnectEvents.OneTapAuthEventsSDK.LOGIN_SUCCESS: // = 'VKSDKOneTapAuthLoginSuccess'
//             // Пользователь успешно авторизовался
//             console.log('мегахорош, ты вошел через вк, ' + e.payload.user.first_name + " " + e.payload.user.last_name + " с вк айди " + e.payload.user.id)
//             console.log('> by Github Copilot: небойся ошибок, они не страшны')

//             // redirect
//             setAuthData(e.payload)
//             return false

//           // Для этих событий нужно открыть полноценный VK ID чтобы
//           // пользователь дорегистрировался или подтвердил телефон
//           case ConnectEvents.OneTapAuthEventsSDK.FULL_AUTH_NEEDED: //  = 'VKSDKOneTapAuthFullAuthNeeded'
//           case ConnectEvents.OneTapAuthEventsSDK.PHONE_VALIDATION_NEEDED: // = 'VKSDKOneTapAuthPhoneValidationNeeded'
//           case ConnectEvents.ButtonOneTapAuthEventsSDK.SHOW_LOGIN: // = 'VKSDKButtonOneTapAuthShowLogin'
//             return Connect.redirectAuth({ url: vkAuthRedirectURL, state: 'from_vk_page'}); // url - строка с url, на который будет произведён редирект после авторизации.
//           // state - состояние вашего приложение или любая произвольная строка, которая будет добавлена к url после авторизации.
//           // Пользователь перешел по кнопке "Войти другим способом"
//           case ConnectEvents.ButtonOneTapAuthEventsSDK.SHOW_LOGIN_OPTIONS: // = 'VKSDKButtonOneTapAuthShowLoginOptions'
//             // Параметр url: ссылка для перехода после авторизации. Должен иметь https схему. Обязательный параметр.
//             return Connect.redirectAuth({ url: vkAuthRedirectURL });
//         }

//         return false;
//       },
//       // Не обязательный параметр с настройками отображения OneTap
//       options: {
//           showAlternativeLogin: false, // Отображение кнопки "Войти другим способом"
//           displayMode: 'name_phone', // Режим отображения кнопки 'default' | 'name_phone' | 'phone_name'
//           buttonStyles: {
//               borderRadius: 8, // Радиус скругления кнопок
//           },
//       },
//     });

//     const vkElementDiv = document.getElementById("vk");
//     vkElementDiv.appendChild(vkOneTapButton.getFrame())
//     vkElementDiv.onClick = () => {
//       setIsLoading(true);
//     }
//     // document.body.appendChild(vkOneTapButton.getFrame())

//     return () => {
//         vkElementDiv.removeChild(vkOneTapButton.getFrame())
//         // document.body.removeChild(vkOneTapButton.getFrame())
//     }
//   }, []);

//   return (
//     <>
//     <div id="vk" className='vk'>
//       {/* <div className="vk-loading">
//           {isLoading && <div className="vk-loading__message" onClick={() => alert(123)}>Загрузка...</div>}
//       </div> */}
//     </div>
//     </>
//   )
// }

import { useState, useEffect } from 'react';
import { Config, Connect, ConnectEvents } from '@vkontakte/superappkit';
import myfetch from '../../FxFetches/myfetch';
import { useDispatch } from 'react-redux';
import { vkDataFetch } from '../../ReduxStates/Slices/vkDataSlice';
import { currentHost } from '../../FxFetches/util';

// Инициализация конфигурации VK
Config.init({
  appId: 51771477, // идентификатор приложения
});

const SERVER_HOST = currentHost;

export default function VkButton() {
  const dispatch = useDispatch();
  const [authData, setAuthData] = useState(null);

  // Загрузка данных VK при монтировании компонента
  useEffect(() => {
    dispatch(vkDataFetch());
  }, []);

  // Обработка данных авторизации VK
  useEffect(() => {
    const vkAuthRedirectURL = SERVER_HOST + '/api/auth/redirect';

    // Функция для обработки событий авторизации
    const handleAuth = (e) => {
      const type = e.type;
      if (!type) return;

      switch (type) {
        case ConnectEvents.OneTapAuthEventsSDK.LOGIN_SUCCESS:
          console.log('VK login success');
          setAuthData(e.payload);
          break;
        case ConnectEvents.OneTapAuthEventsSDK.FULL_AUTH_NEEDED:
        case ConnectEvents.OneTapAuthEventsSDK.PHONE_VALIDATION_NEEDED:
        case ConnectEvents.ButtonOneTapAuthEventsSDK.SHOW_LOGIN:
        case ConnectEvents.ButtonOneTapAuthEventsSDK.SHOW_LOGIN_OPTIONS:
          Connect.redirectAuth({ url: vkAuthRedirectURL });
          break;
        default:
          break;
      }
    };

    // Создание кнопки OneTap VK
    const vkOneTapButton = Connect.buttonOneTapAuth({
      callback: handleAuth,
      options: {
        showAlternativeLogin: false,
        displayMode: 'name_phone',
        buttonStyles: {
          borderRadius: 8,
        },
      },
    });

    const vkElementDiv = document.getElementById("vk");
    vkElementDiv.appendChild(vkOneTapButton.getFrame());

    return () => {
      vkElementDiv.removeChild(vkOneTapButton.getFrame());
    };
  }, []);

  // Обработка авторизационных данных
  useEffect(() => {
    async function authorize() {
      if (!authData) return;
      const vkAuthRedirectURL = '/api/authorize';
      await myfetch(vkAuthRedirectURL, {
        credentials: 'include',
        method: 'POST',
        headers: {
          'Content-Type': 'application/json;charset=utf-8'
        },
        body: JSON.stringify({
          silent_token: authData.token,
          uuid: authData.uuid,
        })
      });
    }

    authorize();
  }, [authData]);

  return (
    <div id="vk" className='vk'></div>
  );
}
