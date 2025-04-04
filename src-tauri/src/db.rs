use directories::BaseDirs;
use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// 创建全局静态连接池（线程安全）
pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let base_dirs = BaseDirs::new().unwrap();
    // Linux:   /home/alice/.config
    // Windows: C:\Users\Alice\AppData\Roaming
    // macOS:   /Users/Alice/Library/Application Support
    // 获取用户数据目录（如：C:\Users\用户名\AppData\Roaming\）
    let data_dir = base_dirs.data_dir().to_path_buf();

    // 拼接数据库路径
    let db_path = data_dir.join("flexo-assistant.db");

    // 连接到SQLite数据库
    let conn = Connection::open(db_path).expect("数据库连接失败");
    Mutex::new(conn)
});

// 自定义错误类型
#[derive(Debug, Serialize, Deserialize)] // 实现序列化特性
pub struct DBError(String); // 使用this error简化错误定义

// 为DBError实现From<rusqlite::Error>
impl From<rusqlite::Error> for DBError {
    fn from(err: rusqlite::Error) -> Self {
        DBError(format!("Database error: {}", err)) // 包装底层错误信息
    }
}
// 为DBError实现From<serde_json::Error>
impl From<serde_json::Error> for DBError {
    fn from(err: serde_json::Error) -> Self {
        DBError(format!("serde_json error: {}", err)) // 包装底层错误信息
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    id: u8,
    theme: bool,                 // 主题：false(明亮)，true(暗黑)
    precision: u8,               // 四舍五入小数位
    k_map: String,               // 用户自定义 k 值
    app_tab: String,             // 父选项卡
    distortion_rate_tab: String, // 子选项卡
    todo_tab: String,            // 子选项卡
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    id: u32,
    name: String,                          // 客户名称
    trimmer_data: Option<Vec<ClientData>>, // 误差微调数据
    fixed_data: Option<Vec<ClientData>>,   // 固定数据
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientData {
    machine: Option<String>,         // 印刷机型号
    width: Option<String>,           // 印刷幅宽
    gears: Option<String>,           // 齿数
    pitch: Option<String>,           // 齿距
    girth: Option<String>,           // 印刷周长
    count: Option<String>,           // 版辊数量
    thickness: Option<String>,       // 版材厚度
    distortion_rate: Option<String>, // 变形率
    trimmer: Option<String>,         // 误差微调
    note: Option<String>,            // 备注
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    date: u64,       // 时间戳
    content: String, // 待办内容
    finished: bool,  // 是否完成
    top: bool,       // 是否置顶
    top_time: u64,      // 置顶时间
    note: bool,      // 是否为笔记
}

// 创建表的函数
fn create_config_table(conn: &mut Connection) -> Result<(), DBError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            id INTEGER PRIMARY KEY,
            theme INTEGER NOT NULL CHECK (theme IN (0, 1)),
            precision INTEGER NOT NULL,
            k_map TEXT NOT NULL,
            app_tab TEXT NOT NULL,
            distortion_rate_tab TEXT NOT NULL,
            todo_tab TEXT NOT NULL
        )",
        (),
    )?;

    // 使用事务确保原子性
    let tx = conn.transaction()?;

    // 检查是否存在数据
    let count: u8 = tx.query_row("SELECT COUNT(*) FROM config", (), |row| row.get(0))?;

    if count == 0 {
        // 插入默认配置
        tx.execute(
            "INSERT INTO config VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (1, &false, &4, &"", &"distortionRate", &"通用", &"全部"),
        )?;
    }
    tx.commit()?; // 结束事务
    Ok(())
}

fn create_client_table(conn: &Connection) -> Result<(), DBError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS client (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            trimmer_data TEXT,
            fixed_data TEXT
        )",
        (),
    )?;
    Ok(())
}

fn create_todo_table(conn: &Connection) -> Result<(), DBError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            date INTEGER NOT NULL,
            content TEXT NOT NULL,
            finished INTEGER NOT NULL CHECK (finished IN (0, 1)),
            top INTEGER NOT NULL CHECK (top IN (0, 1)),
            top_time INTEGER NOT NULL,
            note INTEGER NOT NULL CHECK (note IN (0, 1))
        )",
        (),
    )?;
    Ok(())
}

pub fn init() -> Result<(), DBError> {
    let mut conn = DB.lock().unwrap();
    // 创建表
    create_config_table(&mut conn)?;
    create_client_table(&conn)?;
    create_todo_table(&conn)?;
    Ok(())
}

// 读取用户配置
#[tauri::command]
pub fn get_config() -> Result<Config, DBError> {
    let conn = DB.lock().unwrap();
    let config: Config = conn.query_row("SELECT * FROM config LIMIT 1", (), |row| {
        Ok(Config {
            id: row.get(0)?,
            theme: row.get(1)?,
            precision: row.get(2)?,
            k_map: row.get(3)?,
            app_tab: row.get(4)?,
            distortion_rate_tab: row.get(5)?,
            todo_tab: row.get(6)?,
        })
    })?;
    Ok(config)
}
// 更新用户配置
#[tauri::command]
pub fn update_config(config: Config) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "REPLACE INTO config (id, theme, precision, k_map, app_tab, distortion_rate_tab, todo_tab) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (1, config.theme, config.precision, config.k_map, config.app_tab, config.distortion_rate_tab, config.todo_tab),
    )?;
    Ok(())
}
// 查询所有客户
#[tauri::command]
pub fn get_clients() -> Result<Vec<Client>, DBError> {
    let conn = DB.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM client")?;
    let client_iter = stmt.query_map([], |row| {
        let json_str: String = row.get(2)?; // 使用 String 类型接收
        let trimmer_data: Vec<ClientData> = serde_json::from_str(&json_str).unwrap_or(vec![]);
        let json_str: String = row.get(3)?; // 使用 String 类型接收
        let fixed_data: Vec<ClientData> = serde_json::from_str(&json_str).unwrap_or(vec![]); // 转换为 &str 供 serde 使用
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            trimmer_data: Some(trimmer_data),
            fixed_data: Some(fixed_data),
        })
    })?;
    // 使用filter_map过滤错误并提取Client
    Ok(client_iter.filter_map(|result| result.ok()).collect())
}
// 添加客户信息
#[tauri::command]
pub fn add_client(client: Client) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "INSERT INTO client (name, trimmer_data, fixed_data) VALUES (?1, ?2, ?3)",
        (
            client.name,
            serde_json::to_string(&client.trimmer_data)?,
            serde_json::to_string(&client.fixed_data)?,
        ),
    )?;
    Ok(())
}
// 更新客户信息
#[tauri::command]
pub fn update_client(name: String, data: Vec<ClientData>, fixed: bool) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    if fixed {
        conn.execute(
            "UPDATE client SET fixed_data = ?2 WHERE name = ?1",
            (name, serde_json::to_string(&data)?),
        )?;
    } else {
        conn.execute(
            "UPDATE client SET trimmer_data = ?2 WHERE name = ?1",
            (name, serde_json::to_string(&data)?),
        )?;
    }
    Ok(())
}
// 删除客户信息
#[tauri::command]
pub fn remove_client(name: String) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute("DELETE FROM client WHERE name = ?1", [name])?;
    Ok(())
}
// 查询所有待办
#[tauri::command]
pub fn get_todo() -> Result<Vec<Todo>, DBError> {
    let conn = DB.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            date: row.get(1)?,
            content: row.get(2)?,
            finished: row.get(3)?,
            top: row.get(4)?,
            top_time: row.get(5)?,
            note: row.get(6)?,
        })
    })?;
    // 使用filter_map过滤错误并提取待办
    Ok(todo_iter.filter_map(|result| result.ok()).collect())
}
// 添加待办
#[tauri::command]
pub fn add_todo(todo: Todo) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "INSERT INTO todo (date, content, finished, top, top_time, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (todo.date, todo.content, todo.finished, todo.top, todo.top_time, todo.note),
    )?;
    Ok(())
}
// 更新待办
#[tauri::command]
pub fn update_todo(todo: Todo) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "UPDATE todo SET content = ?2, finished = ?3, top = ?4, top_time = ?5, note = ?6 WHERE date = ?1",
        (todo.date, todo.content, todo.finished, todo.top, todo.top_time, todo.note),
    )?;
    Ok(())
}
// 删除待办
#[tauri::command]
pub fn remove_todo(date: u64) -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute("DELETE FROM todo WHERE date = ?1", [date])?;
    Ok(())
}
// 删除所有已完成待办
#[tauri::command]
pub fn remove_finished_todo() -> Result<(), DBError> {
    let conn = DB.lock().unwrap();
    conn.execute("DELETE FROM todo WHERE finished = 1", ())?;
    Ok(())
}
