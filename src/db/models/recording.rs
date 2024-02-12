table! {
	recordings (id) {
		id -> Int4,
		name -> Varchar,
	}
}

#[derive(Queryable)]
struct Recording {
	id: i32,
	name: String
}

#[derive(Insertable)]
#[table_name="recordings"]
struct NewRecording {
	name: String
}
