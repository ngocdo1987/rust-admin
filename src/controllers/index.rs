use std::collections::HashMap;
use actix_web::{HttpResponse, web::Form, HttpRequest, web};
use fluffy::{tmpl::Tpl, db, model::Model, datetime, utils, random, response,};
use crate::models::{Index as ThisModel, Admins, OSSResult, OSSData, UploadResult};
use std::env;
use actix_session::{Session};
use crate::common::Acl;
use crate::config::{LOGIN_ERROR_MAX, LOGIN_LOCKED_TIME, self};
use crate::caches::admin_roles::ROLE_MENUS;
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;
use std::path::Path;
use std::fs;

//struct Login { 
//    pub ip: String,
//    pub locked_time: usize,
//}
//
//lazy_static! { 
//    pub static ref LOGIN_INFO: HashMap<String, Login> = {
//        HashMap::new()
//    };
//}

pub struct Index {}

impl Index { 
    
    /// Test request
    pub async fn test() -> &'static str { 
        "hello, there"
    }

    /// Backstage homepage login
    pub async fn index(tpl: Tpl) -> HttpResponse { 
        render!(tpl, "index/index.html")
    }

    /// Error page
    pub async fn error(request: HttpRequest, tpl: Tpl) -> HttpResponse { 
        let query_string = request.query_string();
        let queries = fluffy::request::get_queries(&query_string);
        let duration = if let Some(v) = queries.get(&"duration") { if let Ok(n) = v.parse::<usize>() { n } else { 0 } } else { 0 };
        let data = tmpl_data![
            "duration" => &duration,
        ];
        render!(tpl, "index/error.html", &data)
    }

    /// User login
    pub async fn login(session: Session, post: Form<HashMap<String, String>>) -> HttpResponse { 
        // Used to generate the default user/password - start
        //let s1 = random::rand_str(32); //Used to generate the default user password
        //let p1 = utils::get_password("qwe123", &s1); //Default password qwe123
        //println!("UPDATE admins SET secret = '{}', password = '{}' WHERE id = 1", s1, p1);
        // Used to generate the default user/password - end
        //session.remove("locked_time");
        //session.remove("failure_count");
        if let Ok(locked_time) = session.get::<usize>("locked_time") {  //If there is a lock time recorded in the session
            if let Some(n) = locked_time { 
                if (datetime::timestamp() as usize) - n < LOGIN_LOCKED_TIME { 
                    return response::error("Log in too many times, please try again after 2 hours");
                }
            }
        } 

        let mut failure_count = 0_usize; //Login failure number
        if let Ok(failure) = session.get::<usize>("failure_count") {  //Number of failure to log in to log in
            if let Some(n) = failure { 
                failure_count = n; //Number of failure
                if n > LOGIN_ERROR_MAX { 
                    if let Err(message) = session.set::<usize>("locked_time", datetime::timestamp() as usize) { 
                        return response::error(&message.to_string());
                    }
                    return response::error("If there are too many failures, please try again later");
                }
            }
        } else { 
            if let Err(message) = session.set::<usize>("failure_count", failure_count) { 
                return response::error(&message.to_string());
            }
        } //Set the default value of the number of login failure

        if let Err(message) = ThisModel::check_login(&post) {  //If the verification data is wrong
            return response::error(&message);
        }
        
        let name = post.get("username").unwrap();
        let password_ori = post.get("password").unwrap();
        let query = query![fields => "id, password, secret, login_count, role_id",];
        let cond = cond!["name" => &name,];
        let mut conn = db::get_conn();
        if let Some(row) = Admins::fetch_row(&mut conn, &query, Some(&cond)) { 
            let (id, password, secret, login_count, role_id): (usize, String, String, usize, usize) = from_row!(row);
            let password_enc = utils::get_password(password_ori, &secret);
            if password_enc != password {  //Is the password compared to encrypted the same password
                session.set::<usize>("failure_count", failure_count + 1).unwrap();
                return response::error("User name or password error");
            }

            let secret_new = random::rand_str(32);
            let password_new = utils::get_password(&password_ori, &secret_new);
            let now = datetime::timestamp();
            let data = update_row![
                "login_count" => login_count + 1,
                "last_login" => &now,
                "updated" => &now,
                "secret" => &secret_new,
                "password" => &password_new,
            ];
            let cond = cond!["id" => id,];
            if  Admins::update(&mut conn, &data, &cond) == 0 { 
                session.set::<usize>("failure_count", failure_count + 1).unwrap();
                return response::error("Update user information failure");
            }

            session.remove("failure_count"); //Number of failure
            session.remove("locked_time"); //Clear lock time
            session.set::<usize>("user_id", id).unwrap(); //session
            session.set::<String>("user_name", name.to_owned()).unwrap(); //session
            session.set::<usize>("role_id", role_id).unwrap(); //session
            return response::ok();
        } 
        session.set::<usize>("failure_count", failure_count + 1).unwrap();
        response::error("User name or password error")
    }

    /// Exit system
    pub async fn logout(session: Session) -> HttpResponse { 
        session.remove("user_id");
        session.remove("user_name");
        session.remove("role_id");
        response::ok()
    }

    /// change Password
    pub async fn change_pwd(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error?duration=2");
        }
        return render!(tpl, "admins/change_pwd.html");
    }

    /// Save the modification password
    pub async fn change_pwd_save(session: Session, post: Form<HashMap<String, String>>) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::error("Lack of authority");
        }
        if let Err(message) = ThisModel::check_change_pwd(&post) { //Detect the password input correctly
            return response::error(message);
        }
        let password_ori = post.get("old_password").unwrap();
        let user_id = session.get::<usize>("user_id").unwrap().unwrap(); //user ID
        let query = query![fields => "secret, password", ];
        let cond = cond!["id" => user_id, ];
        let mut conn = db::get_conn();
        let row = if let Some(r) = Admins::fetch_row(&mut conn, &query, Some(&cond)) { r }  else { return response::error("Test user information failure"); };
        let (secret, password): (String, String) = from_row!(row);
        if utils::get_password(&password_ori, &secret) != password { 
            return response::error("Old password input error");
        }
        let password_new = post.get("password").unwrap();
        let secret_new = random::rand_str(32);
        let password_enc = utils::get_password(&password_new, &secret_new);
        let data = update_row![
            "password" => &password_enc,
            "secret" => &secret_new,
            "updated" => &datetime::timestamp(),
        ];
        if Admins::update(&mut conn, &data, &cond) == 0 { 
            return response::error("Modify the password failure");
        }
        response::ok()
    }

    /// Background management main interface
    pub async fn manage(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error?duration=2");
        }
        let mut role_id = 0;
        if let Ok(v) = session.get::<usize>("role_id") { 
            if let Some(n) = v { 
                role_id = n;
            }
        }

        let role_menus = &*ROLE_MENUS.lock().unwrap();
        let menus = role_menus.get(&role_id);
        let data = tmpl_data![
            "menus" => &menus,
        ];
        render!(tpl, "index/manage.html", &data)
    }
    
    /// Homepage after entering the background
    pub async fn right(session: Session, tpl: Tpl) -> HttpResponse { 
        if !Acl::check_login(&session) { 
            return response::redirect("/index/error");
        }
        let mut data = tmpl_data![];
        // Current directory
        let current_dir = if let Ok(v) = env::current_dir() { 
            if let Some(p) = v.to_str() { p.to_owned() } else { "".to_owned() }
        } else { "".to_owned() };
        data.insert("current_dir", &current_dir);
        // CPU information
        let cpu_num = if let Ok(n) = sys_info::cpu_num() { n } else { 0 };
        data.insert("cpu_num", &cpu_num);
        // CPU frequency
        let cpu_speed = if let Ok(n) = sys_info::cpu_speed() { n } else { 0 };
        data.insert("cpu_speed", &cpu_speed);
        // Hard disk information
        let disk_info = if let Ok(v) = sys_info::disk_info() { v } else { sys_info::DiskInfo{ total: 0, free: 0 } };
        let disk_info_total = format!("{:.2}", disk_info.total as f64 / (1024. * 1024.));
        let disk_info_free = format!("{:.2}", disk_info.free as f64 / (1024. * 1024.));
        data.insert("disk_info_total", &disk_info_total);
        data.insert("disk_info_free", &disk_info_free);
        // Start Time
        let boot_time_secs = if let Ok(n) = sys_info::boottime() { n.tv_sec as isize } else { 0 };
        let boot_time = format!("{} day {} hour {} sec", boot_time_secs / 86400 , (boot_time_secs % 86400) / 3600, (boot_time_secs % 3600) / 60);
        data.insert("boot_time", &boot_time);
        // Host name
        let host_name = if let Ok(v) = sys_info::hostname() { v } else { "".to_owned() };
        data.insert("host_name", &host_name);
        // 内存信息
        let mem_info = if let Ok(v) = sys_info::mem_info() { (v.total, v.free, v.avail) } else { (0, 0, 0) };
        let mem_info_total = format!("{:.2}", mem_info.0 as f64 / (1024. * 1024.));
        let mem_info_free = format!("{:.2}", mem_info.1 as f64 / (1024. * 1024.));
        let mem_info_avail = format!("{:.2}", mem_info.2 as f64 / (1024. * 1024.));
        data.insert("mem_info_total", &mem_info_total);
        data.insert("mem_info_free", &mem_info_free);
        data.insert("mem_info_avail", &mem_info_avail);
        // operating system
        let os_type = if let Ok(v) = sys_info::os_type() { v } else { "".to_owned()  };
        data.insert("os_type", &os_type);
        // Operating system version
        let os_version = if let Ok(v) = sys_info::os_release() { v } else { "".to_owned() } ;
        data.insert("os_version", &os_version);
        // Number of processes
        let process_count = if let Ok(n) = sys_info::proc_total() { n } else { 0 };
        data.insert("process_count", &process_count);
        // load
        let avg = if let Ok(v) = sys_info::loadavg() { (v.one, v.five, v.fifteen) } else { (0., 0., 0.) };
        data.insert("avg_1", &avg.0);
        data.insert("avg_2", &avg.1);
        data.insert("avg_3", &avg.2);
        let my_version = env!("CARGO_PKG_VERSION");
        data.insert("my_version", &my_version);
        render!(tpl, "index/right.html", &data)
    }
    
    /// Get the address of OSS uploading pictures
    /// ```
    /// {
	///     "code": 0,
	///     "success": true,
	///     "msg": "签名成功",
	///     "data": {
	/// 	    "accessid": "XXXXX",
	/// 	    "host": "http://XXXXX.oss-cn-shanghai.aliyuncs.com",
	/// 	    "policy": "XXXX==",
	/// 	    "signature": "XXXX=",
	/// 	    "expire": 1554851252
	///     }
    /// }
    /// ```
    pub async fn oss_signed_url() -> HttpResponse { 
        let setting = &*config::SETTING;
        let info = &setting.oss;
        let client = oss::OSSClient::new(&info.end_point, &info.access_key_id, &info.access_key_secret);
        let key = "hello";
        let policy_url = client.generate_signed_put_url(&info.bucket, &key, 3600);
        let query_arr = policy_url.split("&").collect::<Vec<&str>>();
        let mut signature = String::new();
        for query in query_arr { 
            let kv = query.split("=").collect::<Vec<&str>>();
            if kv.len() == 2 && kv[0] == "Signature" { 
                signature = kv[1].to_owned();
                break;
            }
        }
        let now = datetime::timestamp();
        let data = OSSData { 
            access_id: &info.access_key_id,
            host: &info.end_point,
            policy: &policy_url,
            signature: &signature,
            expire: now + 3600,
        };
        let result = OSSResult{
            code: 0,
            success: true,
            msg: "成功",
            data,
        };
        HttpResponse::Ok().json(result)
    }

    /// Upload file: Picture
    pub async fn upload_images(payload: Multipart) -> HttpResponse { 
       Self::upload_files(&config::UPLOAD_IMAGE_TYPES, payload).await
    }

    /// upload files
    async fn upload_files(file_types: &[&str],  mut payload: Multipart) -> HttpResponse { 
        let upload_result = |code: u32, message: &str, path: &str| { 
            let result = UploadResult{code: code as usize, message, path};
            HttpResponse::Ok().json(result)
        };
        let upload_error = |code: u32, message: &str| {  //Upload successfully
            upload_result(code, message, "")
        };
        let upload_success = |path: &str| {  //Upload failure and return
            upload_result(0, "", path)
        };
        while let Some(item) = payload.next().await { 
            let mut field = if let Ok(v) = item { v } else { return upload_error(401, "Obtain the upload file failed"); };
    
            // Detect file type, you can only upload picture types
            let mime = field.content_type();
            let file_type = mime.type_().as_str();
            let file_ext = mime.subtype().as_str();
            let file_mine = format!("{}/{}", file_type, file_ext);
            if !file_types.contains(&file_mine.as_str()) { 
                return upload_error(4011, "The upload is not the legal picture file");
            }

            // The processing file is stored by the date directory
            let save_path = Path::new("./public/upload");
            if !save_path.is_dir() { 
                return upload_error(4012, "The upload file directory does not exist");
            }
            let (year, month_, day_) = datetime::date();
            let month = if month_ > 9 { month_.to_string() } else { format!("0{}", month_) }; //Zero before: month
            let day = if day_ > 9 { day_.to_string() } else { format!("0{}", day_) }; //Zero before: Day
            // Determine the directory (year)
            let save_year = format!("{}/{}", save_path.display(), year);
            let save_year_path = Path::new(&save_year);
            if !save_year_path.is_dir() { 
                if let Err(_) = fs::create_dir(&save_year_path) { 
                    return upload_error(line!(), "Failure to create a directory (year)");
                }
            }
            // Determine the directory (month)
            let save_month = format!("{}/{}", save_year_path.display(), month);
            let save_month_path = Path::new(&save_month);
            if !save_month_path.is_dir() { 
                if let Err(_) = fs::create_dir(&save_month_path) { 
                    return upload_error(line!(), "Failure to create a directory (month)");
                }
            }
            // Determine the directory (day)
            let save_day = format!("{}/{}", save_month_path.display(), day);
            let save_day_path = Path::new(&save_day);
            if !save_day_path.is_dir() { 
                if let Err(_) = fs::create_dir(&save_day_path) { 
                    return upload_error(line!(), "Failure to create a directory (day)");
                }
            }
            let save_file_name = format!("{}.{}", random::rand_str(16), file_ext); //The name of the file to be saved
            let save_file_path = format!("{}/{}", save_day_path.display(), save_file_name); //Saved file path

            //let content_type = if let Some(v) = field.content_disposition() { v } else { return upload_error(402, "Get the upload file information error"); };
            //let file_name = if let Some(v) = content_type.get_filename() { v } else { return upload_error(403, "Get the name of upload file failure"); };
            let file_url = format!("/upload/{}/{}/{}/{}", year, month, day, save_file_name);
            //let file_path = dbg!(format!("./public/upload/{}", file_name));
            let mut f = if let Ok(v) = web::block(|| std::fs::File::create(save_file_path)).await { v } else { return upload_error(405, "创建临时文件失败"); };
            while let Some(chunk) = field.next().await { 
                let data = if let Ok(v) = chunk { v } else { return upload_error(406, "Reading file information failed"); };
                f = if let Ok(v) = web::block(move || f.write_all(&data).map(|_| f)).await { v } else { return upload_error(408, "Save the file information failed"); };
            }
            return upload_success(&file_url);
        }
        upload_error(4409, "Upload file failure")
    }
}

