### Аутентифікація

Для доступу до API необхідно використовувати персональний API токен. Заповніть [форму](https://alerts.in.ua/api-request) і ми надішлемо Вам токен.
Токен - це секрет, будь ласка, не розповсюджуйте його в мережі. Будь-яка діяльність з токеном асоціюється з Вашим проєктом.
Кожен запит має містити токен в параметрах запиту або в HTTP Header.

1. В параметрах запиту:

`https://api.alerts.in.ua/v1/alerts/active.json?token=<YOUR_APP_TOKEN>`

2. В HTTP Header:

`Authorization: Bearer <YOUR_APP_TOKEN>`

`curl -X GET 'https://api.alerts.in.ua/v1/alerts/active.json' -H 'Authorization: Bearer <YOUR_APP_TOKEN>'`
