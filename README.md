# Web Service



## Penjelasan Umum

Ini adalah JSON web service, yg melayani REST API.


## Database
isi dari database "api":

```
api=> create table province (id serial primary key, name varchar(15) not null unique, fullname varchar(25) not null, entry_ts timestamp not null);
CREATE TABLE
api=> insert into province (name, fullname) values ('Jabar', 'Jawa Barat', now());
INSERT 0 1
api=> insert into province (name, fullname) values ('Jateng', 'Jawa Tengah', now());
INSERT 0 1
api=> insert into province (name, fullname) values ('Jatim', 'Jawa Timur', now());
INSERT 0 1
```

```
create table city (
  id serial primary key,
  name varchar(20) not null,
  province_id integer not null references province(id),
  entry_ts timestamp not null)
```

Berikut ini contoh suatu case dg menggunakan enum di postgreSQL
```
api=> create type working_mode as enum('FullTime', 'PartTime', 'Remote');
CREATE TYPE
api=> create type member_skill as enum('Developer', 'Designer', 'NetworkAdmin');
CREATE TYPE
api=> create table member (
api(>   id smallserial primary key,
api(>   name varchar(30) not null,
api(>   mode working_mode not null,
api(>   skills member_skill[] not null);
CREATE TABLE

api=> insert into member (name, mode, skills)
api-> values ('Agus', 'FullTime', '{Developer, Designer}');
INSERT 0 1
api=> select * from member;
 id | name |        skills        
----+------+----------------------
  1 | Agus | {Developer,Designer}
(1 row)
```

## Test
dengan curl :

```curl -w "\n" -i http://127.0.0.1:8081/province/ -H 'Content-Type: application/json' -d '{"name": "Sultra", "fullname": "Sulawesi Tenggara", "id": 0, "entry_ts": "2000-01-01T00:00:00"}'```

Keterangan :  
- id diisi otomatis
- entry_ts diisi otomatis (dalam hal kasus/contoh ini)
