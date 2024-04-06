# Alerts.in.ua API Docs

### Загальні відомості

Cервіс alerts.in.ua відображає інформацію про повітряні тривоги та інші загрози на мапі України. Усі дані про загрози беруться з офіційних джерел, такі як канал “Повітряна тривога”, ОВА, Суспільне, ДСНС, тощо.

Для сторонніх розробників доступний API, який дозволяє використовувати дані сервісу в інших застосунках чи фізичних пристроях.

### Аутентифікація

Для доступу до API необхідно використовувати персональний API токен. Заповніть [форму](https://alerts.in.ua/api-request) і ми надішлемо Вам токен.

Токен - це секрет, будь ласка, не розповсюджуйте його в мережі. Будь-яка діяльність з токеном асоціюється з Вашим проєктом.

Кожен запит має містити токен в параметрах запиту або в HTTP Header.

В параметрах запиту:

```
https://api.alerts.in.ua/v1/alerts/active.json?token=<YOUR_APP_TOKEN>
```

В HTTP Header:

```
  https://api.alerts.in.ua/v1/alerts/active.json

  Authorization: Bearer <YOUR_APP_TOKEN>
```

```
curl -X GET 'https://api.alerts.in.ua/v1/alerts/active.json?token=<YOUR_APP_TOKEN>'
```

```
curl -X GET 'https://api.alerts.in.ua/v1/alerts/active.json' -H 'Authorization: Bearer <YOUR_APP_TOKEN>'
```

### Помилки

Усі методи API можуть повертати помилки. Помилки повертаються в форматі JSON.

| Код | Назва | Опис |
| --- | --- | --- |
| 200 | OK  | Запит виконано успішно |
| 304 | Не змінено | Дані не було змінено |
| 401 | Не авторизовано | Токен API відсутній, неправильний, відкликаний або прострочений. |
| 403 | Заборонено | Ваш IP адрес заблокований або API не доступне в вашій країні. |
| 429 | Забагато запитів | Ліміт запитів на хвилину перевищено |

Приклад відповіді з помилкою:

```
{
  "message": "Error occured"
}
```

### Обмеження

На всі API запити діють обмеження по кількості запитів в секунду.

|     |     |
| --- | --- |
| Soft limit | 8-10 запитів в хвилину з однієї IP адреси |
| Hard limit | 12 запитів в хвилину з однієї IP адреси |

При перевищенні ліміту сервер буде повертати код `429 Too many requests` . При систематичному порушені IP та token будуть заблоковані.

Також при аномальній кількості запитів на день, Ваш токен може бути заблокований.

**Використання у публічних сервісах**

Для публічних сервісів використовуйте свій сервер для проксування запитів. Тобто кінцевий користувач повинен отримувати дані через Ваш проксі, а не напряму з alerts.in.ua API. Не харкодьте токен в коді, що отримує користувач (мобільний застосунок, веб-сторінка). Вийняток становлять фізичні пристрої, які не можуть використовувати проксі.

**Кешування**

Для кешування запитів можете використовувати хедер If-Modified-Since з Last-Modified для усіх API. В цьому випадку Ви зможете завантажувати дані тільки коли були здійснені зміни.

### Відмова від відповідальності

Не використовуйте API для критичної інфраструктури.

Доступ до API, а також використання його вмісту здійснюються виключно на Ваш розсуд і на Ваш ризик. Ні за яких обставин Адміністрація Сервісу не несе відповідальності ні перед якою стороною за будь-якої прямої, непрямої, особливий або інший непрямий збиток в результаті будь-якого використання інформації цього Сервісу.

В роботі сервісу можливі затримки з оновленням інформації пов’язані як з технічними моментами, так і з людським фактором, що можете від нас не залежати.

Ми докладаємо всіх зусиль, щоб сервіс працював безперебійно. Однак, не несемо відповідальності за те, що сервіс став тимчасово недоступний через технічні проблеми. У випадку відомих нам технічних проблем ми повідомляємо про це в наш телеграм канал.

Якщо Ви використовуєте API, Ви автоматично погоджуєтесь з встановленими Правилами і приймаєте всю відповідальність, яка може бути на Вас покладена.

### /v1/alerts/active

Список активних тривог

Повертає список регіонів в яких активна повітряна тривога чи будь-яка інша загроза.

```
# Sync mode
from alerts_in_ua import Client as AlertsClient

alerts_client = AlertsClient(token="YOUR_APP_KEY")
active_alerts = alerts_client.get_active_alerts()
print(active_alerts)
```

```
# Async mode
import asyncio
from alerts_in_ua import AsyncClient as AsyncAlertsClient

async def main():
  alerts_client = AsyncAlertsClient(token="YOUR_APP_KEY")
  active_alerts = await alerts_client.get_active_alerts()
  print(active_alerts)

asyncio.run(main())
```

```
const request = require('request');

const token = "<YOUR_APP_KEY>";
const options = {
  url: 'https://api.alerts.in.ua/v1/alerts/active.json',
    headers: {
    'Authorization': `Bearer ${token}`
  }
};

request.get(options, (error, response, body) => {
    if (error) {
      console.error(error);
      return;
    }
    console.log(JSON.parse(body));
});
```

```
require 'rest_client'
require 'json'

token = "<YOUR_APP_KEY>"
url = 'https://api.alerts.in.ua/v1/alerts/active.json'
headers = { 'Authorization': "Bearer #{token}" }

begin
  response = RestClient.get(url, headers)
  if response.code == 200
    result = JSON.parse(response.body)
    puts result
  else
    puts "Error: #{response.code} - #{response.body}"
  end
rescue RestClient::ExceptionWithResponse => e
  puts "Error: #{e.response.code} - #{e.response.body}"
end
```

```
curl https://api.alerts.in.ua/v1/alerts/active.json?token=YOUR_APP_KEY
```

```
{
  "alerts": [{
    "id": 10,
    "location_title": "Луганська область",
    "location_type": "oblast",
    "started_at": "2022-04-04T16:45:39.000Z",
    "finished_at": null,
    "updated_at": "2022-04-08T08:04:26.316Z",
    "alert_type": "air_raid",
    "location_uid": "16",
    "location_oblast": "Луганська область",
    "location_oblast_uid": "16"
    "location_raion": "Луганський район",
    "notes": "За повідомленям голови ОВА",
    "calculated": true
  }]
}
```

```
{
  "message": "API error. Please contact [email protected] for details."
}
```

### /v1/iot/active\_air\_raid\_alerts\_by\_oblast.json [Edit](cloudcannon:collections/_api/iot_active_air_raid_alerts.yml)

Статус повітряних тривог в областях

Повертає стан повітряних тривог в областях. Компактне API для використання в IoT пристроях.

Результат повертається у вигляді JSON, що містить рядок:

`"ANNNNNNNNNNNANNNNNNNNNNNNNN"`

де:

| Код | Значення |
| --- | --- |
| A   | повітряна тривога активна в усій області |
| P   | часткова тривога в районах чи громадах |
| N   | немає інформації про повітряну тривогу |

Для кожної букви рядка є своя область в наступному порядку:

```2
["Автономна Республіка Крим", "Волинська область", "Вінницька область", "Дніпропетровська область", "Донецька область", "Житомирська область", "Закарпатська область", "Запорізька область", "Івано-Франківська область", "м. Київ", "Київська область", "Кіровоградська область", "Луганська область", "Львівська область", "Миколаївська область", "Одеська область", "Полтавська область", "Рівненська область", "м. Севастополь", "Сумська область", "Тернопільська область", "Харківська область", "Херсонська область", "Хмельницька область", "Черкаська область", "Чернівецька область", "Чернігівська область"]
```

Тобто перша буква в рядку - статус повітряної тривоги в Автономній Республіці Крим, друга - в Волинській області, третя - в Вінницькій області і т.д.

```
  # Sync mode
  from alerts_in_ua import Client as AlertsClient

  alerts_client = AlertsClient(token="YOUR_APP_KEY")
  active_alerts = alerts_client.get_air_raid_alert_statuses_by_oblast()
  print(active_alerts)
```

```
# Async mode
import asyncio
from alerts_in_ua import AsyncClient as AsyncAlertsClient

async def main():
   alerts_client = AsyncAlertsClient(token="YOUR_APP_KEY")
   active_alerts = await alerts_client.get_air_raid_alert_statuses_by_oblast()
   print(active_alerts)

asyncio.run(main())
```

```
  require 'rest-client'
  require 'json'

  # API endpoint
  url = 'https://api.alerts.in.ua/v1/iot/active_air_raid_alerts_by_oblast.json'
  token = "<YOUR_APP_KEY>"

  # Fetch the data from the API
  headers = { 'Authorization': "Bearer #{token}" }

  # Parse the JSON response
  begin
      response = RestClient.get(url, headers)
      if response.code == 200
        data = JSON.parse(response.body)

        # Create a hash mapping oblast names to their corresponding statuses
        oblast_status_map = {}
        oblasts = ["Автономна Республіка Крим", "Волинська область", "Вінницька область", ...]
        data.each_char.with_index do |status, i|
            oblast_status_map[oblasts[i]] = case status
            when 'A'
            'Active'
            when 'P'
            'Partly'
            when 'N'
            'No alerts'
            end
        end
        # Output the oblast status map
        puts oblast_status_map
      else
        puts "Error: #{response.code} - #{response.body}"
      end
  rescue RestClient::ExceptionWithResponse => e
         puts "Error: #{e.response.code} - #{e.response.body}"
  end
```

```
curl https://api.alerts.in.ua/v1/iot/active_air_raid_alerts_by_oblast.json?token=YOUR_APP_KEY
```

```
 "ANNNNNNNNNNNANNNNNNNNNNNNNN"
```

```
["Автономна Республіка Крим", "Волинська область", "Вінницька область", "Дніпропетровська область", "Донецька область", "Житомирська область", "Закарпатська область", "Запорізька область", "Івано-Франківська область", "м. Київ", "Київська область", "Кіровоградська область", "Луганська область", "Львівська область", "Миколаївська область", "Одеська область", "Полтавська область", "Рівненська область", "м. Севастополь", "Сумська область", "Тернопільська область", "Харківська область", "Херсонська область", "Хмельницька область", "Черкаська область", "Чернівецька область", "Чернігівська область"]
```

```
{
  "message": "API error. Please contact [email protected] for details."
}
```

### /v1/iot/active\_air\_raid\_alerts/:uid.json [Edit](cloudcannon:collections/_api/iot_active_air_raid_alerts_uid.yml)

Повертає статус тривоги в вказаній області

###### Parameters

uid

Унікальний ідентифікатор області

Повертає стан тривоги в вказаній області. Компактне API для використання в IoT пристроях.

Результат повертається у вигляді JSON, що містить рядок:

`"A"`

де:

| Код | Значення |
| --- | --- |
| A   | повітряна тривога активна в усій області |
| P   | часткова тривога в районах чи громадах |
| N   | немає інформації про повітряну тривогу |

```
  # Sync mode
  from alerts_in_ua import Client as AlertsClient

  alerts_client = AlertsClient(token="YOUR_APP_KEY")
  alert_status = alerts_client.get_air_raid_alert_status()
  # or alert_status = alerts_client.get_air_raid_alert_status('Луганська область')
  print(alert_status)
```

```
# Async mode
import asyncio
from alerts_in_ua import AsyncClient as AsyncAlertsClient

async def main():
   alerts_client = AsyncAlertsClient(token="YOUR_APP_KEY")
   alert_status = await alerts_client.get_air_raid_alert_status(16)
   # or alert_status = await alerts_client.get_air_raid_alert_status('Луганська область')
   print(alert_status)

asyncio.run(main())
```

```
  require 'rest-client'
  require 'json'

  # API endpoint
  url = 'https://api.alerts.in.ua/v1/iot/active_air_raid_alerts/16.json'
  token = "<YOUR_APP_KEY>"

  # Fetch the data from the API
  headers = { 'Authorization': "Bearer #{token}" }

  # Parse the JSON response
  begin
      response = RestClient.get(url, headers)
      if response.code == 200
        data = JSON.parse(response.body)
        status = case data[0]
        when 'A'
           'Active'
        when 'P'
           'Partly'
        when 'N'
           'No alerts'
        end
        # Output the oblast status map
        puts status
      else
        puts "Error: #{response.code} - #{response.body}"
      end
  rescue RestClient::ExceptionWithResponse => e
         puts "Error: #{e.response.code} - #{e.response.body}"
  end
```

```
curl https://api.alerts.in.ua/v1/iot/active_air_raid_alerts/16.json?token=YOUR_APP_KEY
```

```
 "A"
```

```
{
  "message": "API error. Please contact [email protected] for details."
}
```

### /v1/regions/:uid/alerts/:period.json [Edit](cloudcannon:collections/_api/regions_history.yml)

Повертає історію тривог за певний період

###### Parameters

uid

Унікальний ідентифікатор області

period

Період для якого повертається історія тривог.

Через навантаження на сервери ця функція має окремий ліміт 2 рази на хвилину. І з 20 листопада 2023 цей ліміт буде переглянуто. Не рекомендується використовувати цю функцію в реальному часі.

Повертає список тривог за вказаний період.

| Період | Опис |
| --- | --- |
| month\_ago | місяць від поточної дати |

```
# Sync mode
from alerts_in_ua import Client as AlertsClient

alerts_client = AlertsClient(token="YOUR_APP_KEY")
active_alerts = alerts_client.get_alerts_history(16,period="month_ago")
# or active_alerts = alerts_client.get_alerts_history("Луганська область",period="month_ago")
print(active_alerts)
```

```
# Async mode
import asyncio
from alerts_in_ua import AsyncClient as AsyncAlertsClient

async def main():
  alerts_client = AsyncAlertsClient(token="YOUR_APP_KEY")
  active_alerts = await alerts_client.get_alerts_history(16,period="month_ago")
  # or active_alerts = await alerts_client.get_alerts_history("Луганська область",period="month_ago")
  print(active_alerts)

asyncio.run(main())
```

```
const request = require('request');

const token = "<YOUR_APP_KEY>";
const oblast_uid = "16";
const options = {
  url: 'https://api.alerts.in.ua/v1/regions/${oblast_uid}/alerts/month_ago.json',
    headers: {
    'Authorization': `Bearer ${token}`
  }
};

request.get(options, (error, response, body) => {
    if (error) {
      console.error(error);
      return;
    }
    console.log(JSON.parse(body));
});
```

```
require 'rest_client'
require 'json'

token = "<YOUR_APP_KEY>"
oblast_uid = "16"
url = "https://api.alerts.in.ua/v1/regions/#{oblast_uid}/alerts/month_ago.json"
headers = { 'Authorization': "Bearer #{token}" }

begin
  response = RestClient.get(url, headers)
  if response.code == 200
    result = JSON.parse(response.body)
    puts result
  else
    puts "Error: #{response.code} - #{response.body}"
  end
rescue RestClient::ExceptionWithResponse => e
  puts "Error: #{e.response.code} - #{e.response.body}"
end
```

```
curl https://api.alerts.in.ua/v1/regions/16/alerts/month_ago.json?token=YOUR_APP_KEY
```

```
{
  "alerts": [{
    "id": 10,
    "location_title": "Луганська область",
    "location_type": "oblast",
    "started_at": "2022-04-04T16:45:39.000Z",
    "finished_at": null,
    "updated_at": "2022-04-08T08:04:26.316Z",
    "alert_type": "air_raid",
    "location_uid": "16",
    "location_oblast": "Луганська область",
    "location_oblast_uid": "16"
    "location_raion": "Луганський район",
    "notes": "За повідомленям голови ОВА",
    "calculated": false
  },]
  {
    "id": 9,
    "location_title": "Луганська область",
    "location_type": "oblast",
    "started_at": "2022-03-04T16:45:39.000Z",
    "finished_at": null,
    "updated_at": "2022-03-04T16:45:39.000Z",
    "alert_type": "air_raid",
    "location_uid": "16",
    "location_oblast": "Луганська область",
    "location_oblast_uid": "16"
    "location_raion": "Луганський район",
    "notes": "",
    "calculated": false
  }
}
```

```
{
  "message": "API error. Please contact [email protected] for details."
}
```

### Alert [Edit](cloudcannon:collections/_model/alert.yml)

Сутність, що представляє собою інформацію про тривогу.

| Назва поля | Тип даних | Приклад | Опис |
| --- | --- | --- | --- |
| id  | integer($int64) | 10  | Унікальний ідентифікатор запису |
| location\_title | string($string) | Луганська область | Назва локації |
| location\_type | string | oblast | Тип локації |
|     | enum | \[ oblast, raion, city, hromada, unknown \] | Варіанти типу локації |
| started\_at | string($date-time) | 2022-04-04T16:45:39.000Z | Час початку тривоги |
| finished\_at | string($date-time) | null | Час кінця тривоги |
| updated\_at | string($date-time) | 2022-04-08T08:04:26.316Z | Час останнього оновлення запису в базі |
| alert\_type | string | air\_raid | Тип тривоги |
|     | enum | \[ air\_raid, artillery\_shelling, urban\_fights, chemical, nuclear \] | Варіанти типу тривоги |
| location\_uid | string($int32) |     | Унікальний ідентифікатор локації |
| location\_oblast | string | Луганська область | Область локації |
| location\_raion | string | Луганський район | Район локації |
| notes | string | За повідомленям голови ОВА | Нотатки |
| calculated | boolean | false | Визначає чи час закінчення тривоги прогнозований чи викорстаний реальний час закінчення. |

```
 {
    "id": 10,
    "location_title": "Луганська область",
    "location_type": "oblast",
    "started_at": "2022-04-04T16:45:39.000Z",
    "finished_at": null,
    "updated_at": "2022-04-08T08:04:26.316Z",
    "alert_type": "air_raid",
    "location_uid": "16",
    "location_oblast": "Луганська область",
    "location_oblast_uid": "16"
    "location_raion": "Луганський район",
    "notes": "За повідомленям голови ОВА",
    "calculated": false
  }
```

### Location UID [Edit](cloudcannon:collections/_model/uid.yml)

Унікальний ідентифікатор локації

Кожна локація має свій унікальний ідентифікатор. Тут представлені ідентифікатори областей та міст зі спеціальними статусами.

| UID | Назва області/міста |
| --- | --- |
| 3   | Хмельницька область |
| 4   | Вінницька область |
| 5   | Рівненська область |
| 8   | Волинська область |
| 9   | Дніпропетровська область |
| 10  | Житомирська область |
| 11  | Закарпатська область |
| 12  | Запорізька область |
| 13  | Івано-Франківська область |
| 14  | Київська область |
| 15  | Кіровоградська область |
| 16  | Луганська область |
| 17  | Миколаївська область |
| 18  | Одеська область |
| 19  | Полтавська область |
| 20  | Сумська область |
| 21  | Тернопільська область |
| 22  | Харківська область |
| 23  | Херсонська область |
| 24  | Черкаська область |
| 25  | Чернігівська область |
| 26  | Чернівецька область |
| 27  | Львівська область |
| 28  | Донецька область |
| 29  | Автономна Республіка Крим |
| 30  | м. Севастополь |
| 31  | м. Київ |

```
 {
    "id": 10,
    "location_title": "Луганська область",
    "location_type": "oblast",
    "started_at": "2022-04-04T16:45:39.000Z",
    "finished_at": null,
    "updated_at": "2022-04-08T08:04:26.316Z",
    "alert_type": "air_raid",
    "location_uid": "16",
    "location_oblast": "Луганська область",
    "location_oblast_uid": "16"
    "location_raion": "Луганський район",
    "notes": "За повідомленям голови ОВА",
    "calculated": false
  }
```

## Notes

### Country Boundaries

- https://www.openstreetmap.org/relation/60199#map=6/48.447/31.182
- https://osm-boundaries.com/Download/Submit?apiKey=c37e46a9dae1267a7886c406081d01d6&db=osm20240205&osmIds=-60199&recursive&minAdminLevel=2&maxAdminLevel=2&format=EWKT&srid=4326&landOnly&simplify=100
- https://gis.stackexchange.com/a/221456
  ```
  http://nominatim.openstreetmap.org/search?country=Ukraine&polygon_geojson=1&format=json
  http://nominatim.openstreetmap.org/search?country=Ukraine&polygon_kml=1&format=json
  ```
- https://osmand.net/docs/technical/map-creation/creating-a-country-polygon/
- https://stackoverflow.com/questions/64448089/extract-all-polygons-states-of-a-country
  ```
    [out:json][timeout:25];
    {{geocodeArea:Ukraine}}->.searchArea;
    (
      relation["boundary"="administrative"]["admin_level"="2"](area.searchArea);
    );
    out body;
    >;
    out skel qt;
  ```
- `"boundingbox":["44.1845980","52.3797464","22.1370590","40.2278093"]`