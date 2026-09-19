create table users(
	id uuid primary key, 
	email text not null unique, 
	pass_hash text not null, 
	created_at timestamptz default current_timestamp not null
);

create table files(
	id uuid primary key,
	stored_file_name text not null,
	original_file_name text not null,
	content_type text not null,
	file_size int not null,
	uploaded_at timestamptz not null default current_timestamp,
	user_id uuid not null,
	foreign key (user_id) references users(id)
);
