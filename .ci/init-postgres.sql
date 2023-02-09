create table words (
  id uuid primary key,
  word text not null,
  source text not null,
  pinyin text not null,
  translations text[] not null
);

create index on words (word);
create unique index on words (word, source);