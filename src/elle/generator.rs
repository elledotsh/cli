use php_codegen::class::Class;
use php_codegen::comment::Document;
use php_codegen::data_type::DataType::Array;
use php_codegen::file::File as PhpFile;
use php_codegen::function::Function;
use php_codegen::method::Method;
use php_codegen::parameter::Parameter;
use php_codegen::property::Property;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::SystemTime;

pub fn make_migration_create(table_name: &String) {
    let filename = format!(
        "./database/migrations/{}_create_{}_table.php",
        get_current_time(),
        table_name
    );

    let file_header = PhpFile::new()
        .declare("strict_types", 1)
        .uses("Illuminate\\Database\\Migrations\\Migration")
        .uses("Illuminate\\Database\\Schema\\Blueprint")
        .uses("Illuminate\\Support\\Facades\\Schema");

    let file_data = format!(
        r#"
    {}
    
    return new class extends Migration
    {{
        public function up() : void {{
            Schema::create('{}', function (Blueprint $table) {{
                
            }});
        }}

        public function down() : void {{
            Schema::dropIfExists('{}');
        }}
    }}
    "#,
        file_header, table_name, table_name
    );

    let file_to_write = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(file_to_write);
    f.write_all(file_data.as_bytes())
        .expect("Unable to write data");

    print!("{file_data}");
}

pub fn make_model(model_name: &String) {
    let filename = format!("./app/Models/{}.php", model_name);

    let file_data = PhpFile::new()
        .declare("strict_types", 1)
        .namespaced("App\\Models")
        .uses("Elluminate\\Models\\BaseModel")
        .class(
            Class::new(model_name)
                .extends("BaseModel")
                .method(
                    Method::new("endpoints")
                        .public()
                        .returns(Array)
                        .body("return [];"),
                )
                .method(
                    Method::new("relationships")
                        .public()
                        .returns(Array)
                        .body("return [];"),
                )
                .method(
                    Method::new("scopes")
                        .public()
                        .returns(Array)
                        .body("return [];"),
                ),
        )
        .to_string();

    let file_to_write = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(file_to_write);
    f.write_all(file_data.as_bytes())
        .expect("Unable to write data");

    print!("{file_data}");
}

fn get_current_time() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}
