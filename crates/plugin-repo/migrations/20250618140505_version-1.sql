-- Add migration script here
create table if not exists "plugin"
(
    id varchar(255) not null,
    name varchar(255) not null,
    last_version varchar(255) not null,
    constraint PK_PLUGIN primary key (id),
    constraint CK_PLUGIN_VERSION check (last_version like '%.%.%')
);

create table if not exists "plugin_version"
(
    plugin_id varchar(255) not null,
    version   varchar(255) not null,
    constraint PK_PLUGIN_VERSION primary key (plugin_id, version),
    constraint FK_PLUGIN_VERSION_PLUGIN foreign key (plugin_id) references "plugin" (id),
    constraint CK_PLUGIN_VERSION_VERSION check (version like '%.%.%')
);

create table if not exists "category"
(
    name varchar(255) not null,
    constraint PK_CATEGORY primary key (name)
);

create table if not exists "plugin_category"
(
    plugin_id   varchar(255) not null,
    category_name varchar(255) not null,
    constraint PK_PLUGIN_CATEGORY primary key (plugin_id, category_name),
    constraint FK_PLUGIN_CATEGORY_PLUGIN foreign key (plugin_id) references "plugin" (id),
    constraint FK_PLUGIN_CATEGORY_CATEGORY foreign key (category_name) references "category" (name)
);

create table if not exists "tag"
(
    name varchar(255) not null,
    constraint PK_TAG primary key (name)
);

create table if not exists "plugin_tag"
(
    plugin_id varchar(255) not null,
    tag_name  varchar(255) not null,
    constraint PK_PLUGIN_TAG primary key (plugin_id, tag_name),
    constraint FK_PLUGIN_TAG_PLUGIN foreign key (plugin_id) references "plugin" (id),
    constraint FK_PLUGIN_TAG_TAG foreign key (tag_name) references "tag" (name)
);