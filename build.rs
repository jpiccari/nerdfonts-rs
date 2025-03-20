use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufReader, BufWriter, Error, Write},
    path::Path,
};

const FONT_FILE: &str = "glyphnames.json";
const ENTRY_POINT: &str = "mod.rs";

struct Glyph {
    group: String,
    name: String,
    code_point: char,
}

fn main() {
    println!("cargo::rerun-if-changed={}", FONT_FILE);
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/lib.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(ENTRY_POINT);

    let glphys = load_font_json().unwrap();

    let f_entry = File::create(dest_path).expect("unable to create output file");
    let mut f_entry = BufWriter::new(f_entry);

    for (key, value) in glphys {
        let module = key.to_lowercase();
        let path = Path::new(&out_dir).join(format!("{}.rs", &module));
        let f = File::create(path).unwrap();
        let mut f = BufWriter::new(f);

        for glyph in value {
            writeln!(f, "///{}", key).unwrap();
            writeln!(
                f,
                "pub const {}: char = '{}';",
                glyph.name, glyph.code_point
            )
            .unwrap();
        }

        writeln!(f_entry, "pub mod {};", module).unwrap();
    }
}

fn load_font_json() -> Result<HashMap<String, Vec<Glyph>>, Error> {
    let mut glyphs: Vec<Glyph> = vec![];
    let mut result: HashMap<String, Vec<Glyph>> = HashMap::new();

    {
        let font_file = File::open(FONT_FILE)?;
        let reader = BufReader::new(font_file);
        let json_value: serde_json::Value = serde_json::from_reader(reader).unwrap();
        let glyph_json = json_value.as_object().unwrap().to_owned();

        for (key, value) in glyph_json {
            if key == "METADATA" {
                continue;
            }

            let code_point = value.as_object().unwrap()["char"]
                .as_str()
                .unwrap()
                .chars()
                .nth(0)
                .unwrap();

            let (group, name) = key.split_once('-').unwrap();

            glyphs.push(Glyph {
                group: group.to_owned(),
                name: to_const(format!("{}_{}", group, name)),
                code_point,
            });
        }
    }

    for glyph in glyphs {
        let key = glyph.group.clone();
        result.entry(key).or_default().push(glyph);
    }

    Ok(result)
}

fn to_const(name: String) -> String {
    name.to_uppercase().replace("#", "_")
}
