create
database axum_example_db;

-- 票据表
create table tickets
(
    id          char(19) PRIMARY KEY not null, -- 票据id
    user_id     char(19)             not null, -- 票据所属用户的id
    title       varchar(50)          not null, -- 票据头
    create_time timestamp            not null  -- 创建时间
);

-- 用户表
create table users
(
    id          char(19) PRIMARY KEY not null,  -- 用户id
    account     varchar(50)          not null,  -- 用户账号
    password    varchar(50)          not null,  -- 用户密码
    nickname    varchar(50)          not null,  -- 昵称
    role_id     char(19)             not null,  -- 用户所属的角色id
    create_time timestamp            not null,  -- 用户创建时间
    foreign key (role_id) references roles (id) -- 设置与roles表的关联
);

create table roles
(
    id          char(19) PRIMARY KEY not null, -- 角色
    role_name   varchar(50)          not null, -- 角色名称
    create_time timestamp            not null  -- 角色创建时间
);


-- 使用 sea-orm-cli 生成entity内容 ： sea-orm-cli generate entity -u postgres://postgres:12345678@localhost/axum_example_db -o axum-example/src/entity