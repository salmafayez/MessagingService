use rbatis::{crud, impl_delete, impl_select, impl_select_page, impl_update, Rbatis};
use rbdc_oracle::driver::OracleDriver;
use rbdc_oracle::options::OracleConnectOptions;
use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CvmUser {
    pub name: String,
    pub phone: String,
    pub segment: String,
    pub balance: i32
}

crud!(CvmUser{});
impl_select!(CvmUser{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});

pub async fn connect( ) -> Vec<CvmUser>{
    let mut rb = Rbatis::new();
    rb.init_opt(
        OracleDriver {},
        OracleConnectOptions {
            username: "myhrdbuser".to_string(),
            password: "kKLGTFWuvDqhzp3YkNNPbze8fzQgNY".to_string(),
            connect_string: "//myhr-orange-db.cjeeza0z4rpi.us-west-2.rds.amazonaws.com/orngdb".to_string(),
        },
    ).expect("Error Connection");

    let select_result = CvmUser::select_all(&mut rb).await.expect("query failed");
    println!("{:?}",select_result);
    select_result
}