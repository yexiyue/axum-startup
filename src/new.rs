use std::{os::unix::process::CommandExt, process::Command, str::FromStr};

use dialogue_macro::Asker;

#[derive(Debug, Asker)]
struct NewProject {
    #[input(prompt = "Please enter the project name", default = "axum-web")]
    name: String,
    #[confirm(prompt = "do you want to use shuttle?", default = false)]
    use_shuttle: bool,
    #[select(prompt = "which orm do you want to use?", default = 0)]
    orm: Orm,
    #[confirm(
        prompt = "do you want to use logs (tracing,trancing-subscriber)?",
        default = true
    )]
    logs: bool,
}

impl NewProject {
    fn create_project(&self) {
        let mut template_prefix = "axum".to_string();
        if self.use_shuttle {
            template_prefix = "shuttle".to_string();
        }
        match self.orm {
            Orm::Mongodb => template_prefix += "-mongodb",
            Orm::Prisma => template_prefix += "-prisma",
        }
        let path = "templates/".to_string() + &template_prefix;
        let mut features: Vec<&str> = vec![];
        features.push("-d");
        if self.logs {
            features.push("logs=true")
        } else {
            features.push("logs=false");
        }
        tracing::debug!("{:?}", features);
        Command::new("cargo")
            .args(["generate", "yexiyue/axum-startup", &path])
            .arg("-n")
            .arg(&self.name)
            .args(&features)
            .exec();
    }
}

#[derive(Debug, Clone)]
enum Orm {
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
    if !res.use_shuttle.unwrap() {
        options.push(Orm::Prisma);
    }
    let new_project = res.orm(&options).logs().finish();
    new_project.create_project();
}
