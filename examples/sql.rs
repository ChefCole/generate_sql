


#[derive(Debug,generate_sql::GenSql)]
pub struct UserModel{
    pub user_name: String,
    pub password: String
}

fn main(){
    let user_model = UserModel{
        user_name:"名称".to_string(),
        password:"密码".to_string()
    };

    let insert_sql = UserModel::insert_sql(&user_model,"sys_user".to_string());
}