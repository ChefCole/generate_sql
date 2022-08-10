extern crate generate_sql;
use generate_sql::GenSql;


#[derive(Debug,GenSql)]
struct SqlGen{
    id:String,
    nick_name:Option<String>,
    age:i32,
    height:Option<f32>,
    create_time:Option<chrono::NaiveDateTime>
}

fn main() {
    let _model = SqlGen{
        id:"777777".to_string(),
        nick_name:Some("昵称".to_string()),
        age:18,
        height:Some(15.6),
        create_time:Some(chrono::NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11))
    };

    let insert_sql = SqlGen::insert_sql(&_model,"sys_user".to_string());
    let update_sql = SqlGen::update_sql(&_model, "sys_user".to_string(), "id".to_string());
    let select_sql = SqlGen::select_sql(&_model,"sys_user".to_string());
    let delete_sql = SqlGen::delete_sql(&_model, "sys_user".to_string(), "id".to_string());
    println!("{}",insert_sql);
    println!("{}",update_sql);
    println!("{}",select_sql);
    println!("{}",delete_sql);


}
