# Poster
```
docker build -t poster .
docker run -p 8080:8080 -t poster
```

## Api
- [GET /](http://localhost:8080/)
- [POST /register](http://localhost:8080/register)
- [POST /post](http://localhost:8080/post)
- [POST /login](http://localhost:8080/login)
- [POST /page](http://localhost:8080/page)



## Задание
Тестовое задание такое:

Разработать минимальную доску объявлений. 

Функционал: регистрация, логин.

Залогиненный пользователь может оставлять текстовые объявления.

Все пользователи могут их читать (по n штук на страницу, с возможностью переключаться между страницами).

Стек технологий: actix.rs, mongodb

Также нужно предоставить конфиги для docker, которые локально деплоят все.

какой-то rich фронтенд не надо, html без верстки подойдет
