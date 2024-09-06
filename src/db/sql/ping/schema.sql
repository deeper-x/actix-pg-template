CREATE TABLE ping (
	id  BIGSERIAL PRIMARY KEY,
	value text,
	ts_created timestamp default now()
);

