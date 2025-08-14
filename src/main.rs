use std::env;
use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

fn create_file_if_not_exists(path: &PathBuf, content: &str) {
    match File::options().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error when writing file {}: {}", path.display(), e);
            } else {
                println!("Create file {}", path.display());
            }
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("Exist {}", path.display());
        }
        Err(e) => {
            eprintln!("Error when creating file {}: {}", path.display(), e);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: create_subapps <app1> <app2> ...");
        return;
    }

    let current_dir = env::current_dir().expect("Cannot get current directory");
    println!("Base folder: {}", current_dir.display());

    let init_content = "";
    let models_content = "from django.db import models\n\n# Create your models here.\n";
    let serializers_content =
        "from rest_framework import serializers\n\n# Create your serializers here.\n";
    let views_content = "from rest_framework import viewsets\n\n# Create your views here.\n";
    let urls_content = "from django.urls import path, include\nfrom rest_framework.routers import DefaultRouter\n\nrouter = DefaultRouter()\n\nurlpatterns = [\n    path('', include(router.urls)),\n]\n";

    for app_name in args {
        let app_path = current_dir.join(&app_name);

        if let Err(e) = fs::create_dir_all(&app_path) {
            eprintln!("Error when creating file {}: {}", app_path.display(), e);
            continue;
        }

        let files = vec![
            ("__init__.py", init_content),
            ("models.py", models_content),
            ("serializers.py", serializers_content),
            ("views.py", views_content),
            ("urls.py", urls_content),
        ];

        for (file_name, content) in files {
            let file_path = app_path.join(file_name);
            create_file_if_not_exists(&file_path, content);
        }

        println!("Created the subapp: {}\n", app_name);
    }
}
