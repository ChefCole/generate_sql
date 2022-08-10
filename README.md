# generate_sql

### 快速生成sql语句（根据传入struct，自动生成sql语句）

#### 引入
```
generate_sql = "0.1.2"
```
#### 编写struct，添加GenSql
```
#[derive(Debug,GenSql)]
struct SqlGen{
    id:String,
    nick_name:Option<String>,
    age:i32,
    height:Option<f32>,
    create_time:Option<chrono::NaiveDateTime>
}
```
#### 使用方法 (这里时间目前只支持chrono::NaiveDate)

```
fn main() {
    let _model = SqlGen{
        id:"id".to_string(),
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
```
#### 输出结果
```
insert into sys_user (nick_name,age,id,height,create_time) values ('昵称',18,'777777',15.6,'2016-07-08 09:10:11')
update sys_user set nick_name='昵称',age=18,height=15.6,create_time='2016-07-08 09:10:11' where id='777777'
select * from sys_user where nick_name='昵称' and age=18 and id='777777' and height=15.6 and create_time='2016-07-08 09:10:11'
delete from sys_user where id='777777'
```