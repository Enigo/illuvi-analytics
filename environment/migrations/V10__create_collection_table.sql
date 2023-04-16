CREATE table collection
(
    address varchar(255) PRIMARY KEY,
    name varchar(255),
    description varchar(500),
    icon_url varchar(255),
    collection_image_url varchar(255),
    project_id integer,
    project_owner_address varchar(255),
    metadata_api_url varchar(255),
    created_on timestamp,
    updated_on timestamp
);