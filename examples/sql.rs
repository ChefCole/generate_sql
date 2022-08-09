extern crate generate_sql;
use generate_sql::AnswerFn;


#[derive(Debug,AnswerFn)]
struct SqlGen{
    id:Option<String>,
    user:Option<String>,
    password:Option<String>,
    age:Option<i32>,
    money:Option<f32>,
    create_time:Option<chrono::NaiveDateTime>
}

fn main() {
    let _model = SqlGen{
        id:Some("121212121212".to_string()),
        user:Some("123".to_string()),
        password:Some("122".to_string()),
        age:None,
        money:Some(12.4),
        create_time:Some(chrono::NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11))
    };

    let insert_sql = SqlGen::insert_sql(_model,"sys_user".to_string());
    println!("{}",insert_sql);


}
