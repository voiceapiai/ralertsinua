CREATE TABLE IF NOT EXISTS regions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    a_id INTEGER NOT NULL,
    osm_id INTEGER NOT NULL UNIQUE,
    name TEXT NOT NULL,
    name_en TEXT NOT NULL,
    UNIQUE(a_id) ON CONFLICT IGNORE
);

CREATE TABLE IF NOT EXISTS geo (
  osm_id INTEGER NOT NULL UNIQUE,
  geo TEXT NOT NULL,
  FOREIGN KEY(osm_id) REFERENCES regions(osm_id)
);

CREATE TABLE IF NOT EXISTS statuses (
  timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  status TEXT NOT NULL CHECK(length(status) = 27)
);

INSERT INTO regions (osm_id,a_id,name,name_en) VALUES
(145279,29,'Автономна Республіка Крим','Autonomous Republic of Crimea'),
(142129,4,'Волинська область','Volyn Oblast'),
(181453,8,'Вінницька область','Vinnytsia Oblast'),
(203493,9,'Дніпропетровська область','Dnipropetrovsk Oblast'),
(143947,28,'Донецька область','Donetsk Oblast'),
(142491,10,'Житомирська область','Zhytomyr Oblast'),
(144979,11,'Закарпатська область','Zakarpattia Oblast'),
(143961,12,'Запорізька область','Zaporizhia Oblast'),
(144977,13,'Івано-Франківська область','Ivano-Frankivsk Oblast'),
(843733,31,'Київ','Kyiv'),
(142497,14,'Київська область','Kyiv Oblast'),
(203719,15,'Кіровоградська область','Kirovohrad Oblast'),
(143943,16,'Луганська область','Luhansk Oblast'),
(144761,27,'Львівська область','Lviv Oblast'),
(145271,17,'Миколаївська область','Mykolaiv Oblast'),
(145269,18,'Одеська область','Odesa Oblast'),
(182589,19,'Полтавська область','Poltava Oblast'),
(142473,5,'Рівненська область','Rivne Oblast'),
(3148729,30,'Севастополь','Sevastopol'),
(142501,20,'Сумська область','Sumy Oblast'),
(145051,21,'Тернопільська область','Ternopil Oblast'),
(142509,22,'Харківська область','Kharkiv Oblast'),
(142045,23,'Херсонська область','Kherson Oblast'),
(181485,3,'Хмельницька область','Khmelnytskyi Oblast'),
(182557,24,'Черкаська область','Cherkasy Oblast'),
(145053,26,'Чернівецька область','Chernivtsi Oblast'),
(142499,25,'Чернігівська область','Chernihiv Oblast');
