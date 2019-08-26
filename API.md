### Organisation Functions 
```

register_new_organisation (
	user: {
		full_name,
		job_title,
		email,
		profile_picture,
	},
	organisation: {
		org_name,
		org_picture,
		{admin_info},
	},
	members: [
		{
			full_name,
			email,
			job_title,
		},
	],
) -> ok

get_organisation_info (auth_token) -> {org_info}

modify_organisation_info (auth_token, {org_info}) -> ok

```

### Common Entity functions 

```

get_entity_history (auth_token, entity_id) -> [
	[date, action, filename, url, folder, username, user_id],
]

get_entity_files (auth_token, entity_id) -> {
	files: [files: {filename, {notes}, {metadata}, url}],
	activity_feed: [activity]
}

get_entity_notes (auth_token, entity_id) -> [
	[user_name, user_id, date, note, note_id],
]

delete_entity (auth_token, entity_id) -> ok

upload_files_to_entity (auth_token, entity_id, [files]) -> ok

delete_files_from_entity (auth_token, entity_id, [files]) -> ok

create_note_on_file (auth_token, entity_id, file_id, note) -> ok

create_entity_note (auth_token, entity_id, note) -> ok

delete_entity_note (auth_token, note_id) -> ok

```

### Owner functions 

```

get_owners (
	auth_token
) -> [
	{admin_info}
]


create_new_owner (
	auth_token,
	same_as_manager
	{admin_info}
) -> ok

modify_owner_info (
	auth_token,
	same_as_manager
	{admin_info}
) -> ok

get_owner_info (
	auth_token,
	owner_id,
) -> {
	{profile_info}
	{buildings},
	{users},
	
}

```

### User functions 

```

login (
	email,
	password
) -> ok


logout (auth_token) -> ok

get_users (auth_token, owner_id) -> [{user_info}, ]

create_new_user (
	auth_token,
	full_name,
	job_title,
	email,
) -> ok

get_user_profile () -> {user_profile}

modify_user_profile (auth_token, {user_profile}) -> ok

```

### Register functions 

```

create_new_register (auth_token, building_id, register_name) -> ok

```

### Building functions 

```

create_new_building (
	auth_token,
	building_name,
	owner,
	manager,
	respondant,
	adress,
	picture,
	{construction},
	{fire_resistance},
	{alarm_system},
	{sprinkler_system},
	{special_elements},
	{building_history: []},
	{different_measures: []},	
) -> ok

modify_building (
	auth_token,
	building_id,
	building_name,
	owner,
	manager,
	respondant,
	adress,
	picture,
	{construction},
	{fire_resistance},
	{alarm_system},
	{sprinkler_system},
	{special_elements},
	{building_history: []},
	{different_measures: []},	
) -> ok

get_building_information (auth_token, building_id) -> (
	building_name,
	owner,
	manager,
	respondant,
	adress,
	picture,
	{construction},
	{fire_resistance},
	{alarm_system},
	{sprinkler_system},
	{special_elements},
	{building_history: []},
	{different_measures: []},
	{registers: []},
)

```

### Search functions 

```

do_search (auth_token, search_str) -> [{result}, ]

```
