<span style="color: red">Note: Full-stack Rust-Admin 2.0 is under development. The current version is only maintained. Please add WeChat group to communicate </span>

Rust-Admin 2.0 Introduction:

1. Development of the front -end using yew/WASM framework

2. Use Actix-WEB 3.x back end

3. UI uses a paid -authorized version of LAYUI to adjust and optimize (thanks to netizens TARO's enthusiastic support, thank you community)

4. The database adjustment is adjusted to postgresql, and the connection pool management uses SQLX, the database is superior

5. End and back -end separation, data transmission encryption

6. Congenital support distributed deployment

Quantity, please look forward to it.Thank you for your support !!!


# Rust -based background management system

## Features
#### The front end is based on X-Admin and LAYUI. There are many users and easy to modify.

X-Admin: http://x.xuebingsi.com/

Layui: https://www.larryms.com/

#### The back-end is based on Actix-WEB, and the performance tests are slaughtered all year round.

Actix framework: https://actix.rs/

Performance Testing: https://www.techempower.com/benchmarks/

#### MVC design mode, quickly get started, easy to get started.

#### TERA template engines, layout, elements and other concepts are simplified and developed. 

Tera: https://tera.netlify.com/docs/

#### Based on Rust language characteristics, performance and security guarantee

## Secondary development & technology exchange
#### WeChat group, scan code remarks: 'rust', otherwise it will not pass
![avatar](/public/wx.jpeg)


## Environmental requirements
rust: 1.40+ / Mysql: 5.6+ / Nginx: 1.0+ (可选, 如果通过域名/80端口代理方式访问则需要)

## Directory description
#### /public Used to set the website address of Nginx outside
#### /scripts SQL script used to initialize
#### /src Rust source code
#### /setting.toml.default The default configuration file, please copy it to setting.toml and add ignoring
#### /templates Template file
#### /nginx.conf.default Set the configuration file of nginx as the front -end proxy (optional)

## Interface load
#### login interface
![avatar](/public/static/images/login.png)

#### Backstage management
![avatar](/public/static/images/right.png)

#### Menu management
![avatar](/public/static/images/menus.png)

## Instructions for use
#### Download code

```bash
git clone https://gitee.com/houhanting/rust-admin.git
cd rust-admin
```

#### Create a database (MySQL) and enter the guidance data

```sql
CREATE DATABASE rust_admin -- Create a database
    DEFAULT CHARSET=UTF8 
    COLLATE=UTF8_GENERAL_CI; 
GRANT ALL PRIVILEGES -- Set the user name password
    ON `rust_admin`.* 
    TO 'rust_admin'@'%' 
    IDENTIFIED BY 'rust-x-lsl'; 
FLUSH PRIVILEGES;
USE rust_admin; -- Select database
SOURCE scripts/init.sql; -- Import initialization database (please follow the actual path)

-- The following is not necessary, only the front-end use Rust-Vlog
CREATE DATABASE rust_vlog -- Create a VLOG sample database
    DEFAULT CHARSET=UTF8 
    COLLATE=UTF8_GENERAL_CI; 
GRANT ALL PRIVILEGES -- Set the vlog user name password
    ON `rust_vlog`.* 
    TO 'rust_vlog'@'%' 
    IDENTIFIED BY 'rust-x-lsl'; 
FLUSH PRIVILEGES;
USE rust_vlog; -- Select VLOG database
SOURCE scripts/example-vlog.sql; -- Import the initialized VLOG database (please follow the actual path)
```

***** * Default user/name: admin / qwe123

#### Set Nginx proxy (non -required)

Set and generate Nginx configuration file
```bash
cp nginx.conf.default nginx.conf #Copy nginx configuration file
cat "/nginx.conf" >> .git/info/exclude #Ignore nginx configuration file
vim nginx.conf #Modify the corresponding domain name, directory, proxy address, port
```

#### Runtime

```bash
cargo run #Production mode: cargo run --release
```

