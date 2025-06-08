create table if not exists account
(
    id       int          not null auto_increment,
    email    varchar(255) not null,
    password varchar(255) not null,
    constraint PK_ACCOUNT primary key (id)
);

create table if not exists license
(
    id varchar(255) not null,
    max_devices int not null default 2,
    end_date timestamp not null default current_timestamp + interval 1 month,
    constraint PK_LICENSE primary key (id)
);

create table if not exists account_license
(
    account_id int not null,
    license_id varchar(255) not null,
    constraint PK_ACCOUNT_LICENSE primary key (account_id, license_id),
    constraint FK_ACCOUNT_LICENSE_ACCOUNT foreign key (account_id) references account (id),
    constraint FK_ACCOUNT_LICENSE_LICENSE foreign key (license_id) references license (id)
);

create table if not exists app_installation
(
    uuid varchar(45) not null,
    version varchar(255) not null,
    license_id varchar(255) null,
    last_used_at timestamp not null default current_timestamp on update current_timestamp,
    constraint PK_APP_INSTALLATION primary key (uuid),
    constraint FK_APP_INSTALLATION_LICENSE foreign key (license_id) references license (id)
);

create table if not exists remote_supp_request
(
    id int not null auto_increment,
    uuid varchar(45) not null,
    created_at timestamp not null default current_timestamp,
    status varchar(20) not null default 'PENDING',
    constraint PK_REMOTE_CONNECTION_REQUEST primary key (id),
    constraint FK_REMOTE_CONNECTION_REQUEST_APP_INSTALLATION foreign key (uuid) references app_installation (uuid),
    constraint CK_REMOTE_CONNECTION_REQUEST_STATUS check (status in ('PENDING', 'ACCEPTED', 'REJECTED', 'CANCELLED', 'FINISHED'))
);