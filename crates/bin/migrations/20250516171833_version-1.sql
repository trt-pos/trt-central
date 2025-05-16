create table account
(
    id       int          not null auto_increment,
    email    varchar(255) not null,
    password varchar(255) not null,
    constraint PK_ACCOUNT primary key (id)
);

create table license
(
    id varchar(255) not null,
    constraint PK_LICENSE primary key (id)
);

create table account_license
(
    account_id int not null,
    license_id varchar(255) not null,
    constraint PK_ACCOUNT_LICENSE primary key (account_id, license_id),
    constraint FK_ACCOUNT_LICENSE_ACCOUNT foreign key (account_id) references account (id),
    constraint FK_ACCOUNT_LICENSE_LICENSE foreign key (license_id) references license (id)
)