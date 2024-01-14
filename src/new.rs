use std::str::FromStr;

use dialogue_macro::Asker;
use handlebars::DirectorySourceOptions;
use serde_json::json;

#[derive(Debug, Asker)]
pub struct NewProject {
    #[input(prompt = "Please enter the project name", default = "axum-web")]
    pub name: String,
    #[confirm(prompt = "do you want to use shuttle?", default = false)]
    pub use_shuttle: bool,
    #[select(prompt = "which orm do you want to use?", default = 0)]
    pub orm: Orm,
}

#[derive(Debug, Clone)]
pub enum Orm {
    Prisma,
    Mongodb,
}

impl FromStr for Orm {
    type Err = String;
    fn from_str(s: &str) -> Result<Orm, Self::Err> {
        match s {
            "prisma" => Ok(Orm::Prisma),
            "mongodb" => Ok(Orm::Mongodb),
            _ => Err("invalid orm".to_string()),
        }
    }
}

impl ToString for Orm {
    fn to_string(&self) -> String {
        match self {
            Orm::Prisma => "prisma".to_string(),
            Orm::Mongodb => "mongodb".to_string(),
        }
    }
}

pub fn create_new_project() {
    let mut res = NewProject::asker();
    res.name().use_shuttle();
    let mut options = vec![Orm::Mongodb];
    if res.use_shuttle.unwrap() {
        options.push(Orm::Prisma);
    }
    let res = res.orm(&options).finish();
    tracing::debug!("create new project {res:?}");

    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(
            "templates/axum-mongodb",
            DirectorySourceOptions {
                tpl_extension: ".rs".to_string(),
                hidden: false,
                temporary: false,
            },
        )
        .unwrap();
    let tpls=handlebars.get_templates();
    let tpls=tpls.keys().collect::<Vec<_>>();
    tracing::debug!("templates:{tpls:?}");
    let res = handlebars
        .render("axum-mongodb", &json!({}))
        .unwrap();
    tracing::info!("render result:{}", res);
}
