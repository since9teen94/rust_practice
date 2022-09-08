create table users (
	id serial primary key,
	first_name varchar(255) not null,
	last_name varchar(255) not null,
	email varchar(255) unique not null,
	password varchar(255) not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);
