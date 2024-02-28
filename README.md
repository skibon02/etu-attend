# etu-schedule

## Что это?
  [https://etu-schedule.ru](https://etu-schedule.ru) - это сторонний сайт, для просмотра расписания и [безопасного](#воруем-ли-мы-ваши-данные) управления посещаемостью в ЛЭТИ. Основным отличием от оригинальной ИС "Посещаемость" является возможность автоматически отмечаться на парах. Мы используем [токен](#как-получить-токен) "ИС Посещаемость", чтобы наш сервер автоматически отмечал вас на ***выбранных*** парах.  
  
  Ещё одной возможностью etu-schedule является возможность выбирать, какие именно пары вы хотите посещать. Вы можете настроить режим посещения во вкладке **планирование**.  
  
  Также, вы можете временно "отклониться" от своего режима - для этого во вкладке **расписание** рядом с каждым предметом есть кнопка "часики" - нажав на нее, вы можете изменить выбранный план только на эту неделю. 
 
 Помимо этого сайт позволяет оставлять [заметки](#заметки) для любой пары.

## Как получить токен?
На самом сайте присутствует инструкция получения токена, дополненная скриншотами с сайта ИС "Посещаемость". Далее представлено более подробное описание. 

+ Чтобы получить токен вам необходимо открыть сам сайт "ИС Посещаемость" и авторизоваться, если вы не были авторизованы.  
+ После этого нажмите клавишу f12 (или правой кнопкой мыши в любом месте страницы и в появившемся меню выберите "inspect") - так в любом браузере вы войдёте в меню разработчика.  
+ Найдите вкладку "Network" в верхних строчках открывшегося меню.
+ Внутри нее в строке ниже выберите вкладку "Fetch/XHR" и обновите страницу. В процессе загрузки вы увидете внутренние запросы сайта на сервер.
+ Вам необходимо нажать на запрос, который называется "chek-in" - откроется информация об этом запросе. Нужна та, что находится во вкладе "Headers".
+ Внутри заголовков запроса найдите параметр Cookie: в самом конце строки будет "connect.digital-attendance=<*ваш_токен*>".

Сам токен представляет из себя символьную последовательность, например, s%3A5HvEakctEXAlGuHcK2VmdmGrUJ1uaEij.Q1pxpeJgR31h948gVzNf0tsmBhwXkeH33jP4uzIPotI. Таким образом из строки "connect.digital-attendance=s%3A5HvEakctEXAlGuHcK2VmdmGrUJ1uaEij.Q1pxpeJgR31h948gVzNf0tsmBhwXkeH33jP4uzIPotI" вы выбираете сам токен и переносите на наш сайт во вкладку **профиль**.

Следующие 6 дней с момента вашей последней авторизации в ИС "Посещемость" (пока не закончится срок жизни токена) наш сервер будет автоматически отмечать вас на указанных вами парах.

## Заметки
Наш сайт также предоставляет возможность оставлять заметки для каждой пары. Существуют заметки двух типов: ваши личные заметки и заметки группы, то есть те, оставлять которые может только староста. *Как сайт поймет, что пользователь является старостой?* По токену. Поэтому старостам, чтобы оставлять заметки группы необходимо сначала ввести свой токен. 

## Воруем ли мы ваши данные?
Нет, мы не получаем никакой информации о вас на основе вашего токена, за исключением разве что того, в какой группе вы состоите и являетесь ли вы старостой, или данных ВК, таких как ваше имя, фамилия и фотография профиля, необходимых для авторизации. Пользуясь etu-schedule, вы не предоставляете нам никакой приватной информации.  

