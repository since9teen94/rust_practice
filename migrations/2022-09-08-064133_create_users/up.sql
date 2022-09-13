create table users (
	id serial primary key,
	first_name varchar(255) not null,
	last_name varchar(255) not null,
	email varchar(255) unique not null,
	password varchar(255) not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);

insert into users (
	first_name,
	last_name,
	email,
	password
) values (
	'Frodo',
	'Baggins',
	'frodo@theshire.com',
	'$argon2id$v=19$m=4096,t=3,p=1$A2uYmfHJZkAQ55CCvpTujA$aBoQLUaRrqIQl33JcKRqy+x7a/WQBpNEsuJJjCUylyk'
)
