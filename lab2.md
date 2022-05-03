# Отчет по лабораторной работе №2

## Цель

Сформировать набор данных, позволяющий производить операции на реальных объемах данных.

## Программа работы

1. Реализация в виде программы параметризуемого генератора, который позволит сформировать набор связанных данных в каждой таблице.
2. Частные требования к генератору, набору данных и результирующему набору данных:
    * Количество записей в справочных таблицах должно соответствовать ограничениям предментой области.
    * Количество записей в таблицах, хранящих информацию об объектах или субъектах должно быть параметром генерации.
    * Значения для внешних ключей необходимо брать из связанных таблиц.
    * Сохранение уже имеющихся данных в базе данных.

## Ход работы

### Выбор инструмента

Для реализации взаимодействия с БД я выбрал библиотеку [Exposed](!https://github.com/JetBrains/Exposed) для Kotlin, в качестве адаптера используется jdbc.

Для генерации данных используется библиотека [Kotlin-Faker](!https://github.com/serpro69/kotlin-faker)

### Метод генерации

>Я сделал небольшое изменение полей в [скрипте](lab1/update_tables.sql) из первой работы, в таблицах **Ticket** и **RouteSections** departure_station и destination_station теперь называются departure_station_id и destination_station_id соответственно. Аналогично для first_station и last_station из **Route**. И еще я переименовал wagon_id на wagon_class_id в **TrainWagons**.

В порядке генерации:

* Содержание таблицы **wagon_classes** я определил вручную, она состоит из 4 элементов (econom, lux, ultra, seat) - классы вагонов, так как предполагается что их число ограничено и невелико.
* Генерация станций производится через kotlin-faker, но географические координаты заполняются случайно.
* Генерация данных пассажиров производится через kotlin-faker, в половине случаев номер телефона окажется пустым. Номер документа имеет вид "PASSPORT NO XXXXXXXXX...", где иксы - это случайное число типа long. Документ должен быть уникальным (хотя мне кажется от этого следовало было бы отказаться, так как в "реальности" могут быть случаи, когда будут указаны чужие данные).
* Генерация маршрутов выполняется одновременно с генерацией участков пути. Выбирается список станций и формируется время отправления/прибытия, при этом временной порядок соблюдается. Кроме того учитывается время на остановку. Условная стоимость маршрута генерируется случайно (и ограничена сверху).
* Генерация состава выполняется одновременно с генерацией вагонов состава. Выбирается случайный маршрут, длина и дата отправления. Затем случайным образом генерируются вагоны (их количество равно длине состава)
* Генерация заказов и билетов производится одновременно. Сначала формируется время формирования заказа, если после этой даты нет составов, то попытка генерации пропускается. Затем выбирается список доступных поездов на эту дату. Для простоты в расчет берется только дата отправления состава. В половине случаев номер телефона окажется пустым. Затем производится несколько попыток на генерацию билетов, для этого нужно сгенерировать номер вагона (с учетом длины состава), затем найти класс вагона в таблице **train_wagons**, с учетом его вместимости сгенерировать номер места. Выбирается пункт отправления и назначения, по ним формуруется время отправления и назначения. После этого определяется, занято ли это место (с учетом времени), если да, то попытка генерации пропускается. В конце определяется стоимость поездки с учетом множителя вагона через суммирование участков пути.

Так как генерация пассажиров реализована отдельно от заказов и билетов, то в конце работы программы удаляются пассажиры, у которых нет билетов. Кроме того удаляются заказы у которых нет билетов (такое может быть, если все доступные места оказались заняты)

Так что в итоге пассажиров и заказов может оказаться меньше чем было указано. Кроме того число маршрутов тоже может быть меньше чем было указано (так как название должно быть уникальным, а оно составляется из названий начальной и конечной станции)

### Параметры генерации

Всего есть 5 параметров:

* Количество новых станций
* Количество новых пассажиров
* Количество новых маршрутов
* Количество новых составов
* Количество новых заказов

Кроме того можно указать другой url.

Количество билетов зависит от количества заказов, билетов на заказ может быть от 1 до 8.

Количество участков пути в маршруте также варьируется от 1 до 8.

Список аргументов представлен ниже, можно вызвать через флаг -h

```text
Usage: railway options_list
Options: 
    --url [jdbc:postgresql://localhost:5432/railway] { String }
    --stationsNumber, -s [0] { Int }
    --passengersNumber, -p [0] { Int }
    --routesNumber, -r [0] { Int }
    --trainsNumber, -t [0] { Int }
    --ordersNumber, -o [0] { Int }
    --help, -h -> Usage info 
```

Все числовые параметры равны 0 по умолчанию.

### Программа

Проект находится в [папке](lab2/data_gen).

Готового jar нет, так как он выходит слишком большым (26 Мб)...

## Выводы

В результате выполнения данной работы я написал программу, генерирующую корректные параметризируемые случайные записи для базы данных.