CREATE TABLE ping (
	id  BIGSERIAL PRIMARY KEY,
	value text,
	ts_created timestamp default now()
);

CREATE TABLE migrations (
	id BIGSERIAL PRIMARY KEY,
	query text,
	ts_created timestamp default now()
);