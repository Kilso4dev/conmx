use std::{
    collections::HashMap,
    path::Path,
    fs::{
        self,
        DirEntry,
    },
    fmt,
    sync::RwLock,
};
use regex::Regex;

use log::{
    error,
    warn,
    info,
};

use serde_json::{
    self,
    Value,
};

use crate::err::ConmxErr;

type LocaleStringMap = HashMap<String, String>;
type LocaleCategories = HashMap<String, LocaleStringMap>;

#[derive(Debug)]
pub struct Localization {
    pub locales: HashMap<String, LocaleCategories>,
    pub locale: String,
    pub default: String,
}


/*
("en_US", "locale/en_US.json"),
("de_DE", "locale/de_DE.json")

        {
            let mut v: Vec<LocaleProto> = Vec::new();
            $(v.push(Locale::new($loc, $path));)+
            v.into_iter()
                .map(|i| i.build())
                .collect()
        },
*/

#[derive(Debug, Clone)]
enum StringVal {
    Str(String),
    Array(Vec<Box<StringVal>>),
    Map(HashMap<String, Box<StringVal>>),
}

impl StringVal {
    pub fn from_json_value(v: &Value) -> Result<Self, ConmxErr> {
        match v {
            Value::String(s) => Ok(StringVal::Str(s.clone())),
            Value::Bool(v) => Ok(StringVal::Str(format!("{}", v))),
            Value::Number(v) => Ok(StringVal::Str(format!("{}", v))),
            Value::Null => Ok(StringVal::Str(String::from("null"))),
            Value::Array(a) => {
                let mut ret_vec = Vec::with_capacity(a.len());
                for c_i in a {
                    ret_vec.push(Box::new(Self::from_json_value(c_i)?));
                }
                Ok(StringVal::Array(ret_vec))
            }
            Value::Object(o) => {
                let mut ret_map = HashMap::with_capacity(o.len());
                for (k, v) in o {
                    ret_map.insert(k.clone(), Box::new(Self::from_json_value(&v)?));
                }
                Ok(StringVal::Map(ret_map))
            }
        }
    }

    fn format_rec(depth: usize, o: &StringVal) -> String {
        let tabs = "\t".repeat(depth);
        match o {
            StringVal::Str(s) => s.clone(),
            StringVal::Array(a) => {
                let mut r = String::new();
                r.push_str("[\n");
                for c_i in a {
                    r.push_str(format!("{}\t{},\n", tabs, Self::format_rec(depth+1, c_i)).as_str());
                }
                r.push_str(format!("{}],\n", tabs).as_str());
                r
            }
            StringVal::Map(m) => {
                let mut r = String::new();
                r.push_str("{\n");
                for (k, v) in m {
                    r.push_str(format!("{}\t{}: {},\n", tabs, k, Self::format_rec(depth+1, v)).as_str());
                }
                r.push_str(format!("{}}},\n", tabs).as_str());
                r
            }
        }
    }
}

impl fmt::Display for StringVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::format_rec(0, self))
    }
}

fn conv_cat(m: &StringVal) -> Result<LocaleStringMap, ConmxErr> {
    match m {
        StringVal::Str(s) => Err(ConmxErr::Locale(format!("Category formatting is not correct (value \"{}\")", s))),
        StringVal::Array(a) => Err(ConmxErr::Locale(format!("Category formatting is not correct ({:?})", a))),
        StringVal::Map(m) => {
            let mut cat: LocaleStringMap = HashMap::with_capacity(m.len());

            for (k, v) in m {
                match *v.to_owned() {
                    StringVal::Str(s) => { cat.insert(k.clone(), s.clone()); },
                    v_val => warn!("Key \"{}\" has a non-valid child {:?}", k, v_val),
                }
            }

            Ok(cat)
        }
    }
}

fn conv_cats(m: &serde_json::Map<String, Value>) -> Result<LocaleCategories, ConmxErr> {
    let mut cats: LocaleCategories = HashMap::with_capacity(m.len());

    for (k, v) in m {
        match StringVal::from_json_value(v) {
            Ok(m) => match conv_cat(&m) {
                Ok(cat) => {
                    cats.insert(k.clone(), cat);
                }
                _ => (),
            }
            _ => (),
        };
    }

    Ok(cats)
}


fn load_locale(fpath: DirEntry) -> Option<(String, LocaleCategories)> {
    let re = Regex::new(r"(.+)\.json").unwrap();
    let fname: String = fpath.file_name().into_string().unwrap();

    match re.captures(&fname) {
        Some(locale) => {
            Some((
                locale.get(1).unwrap().as_str().to_owned(),
                {
                    let file_lines = fs::read_to_string(fpath.path()).ok()?;
                    let parsed: Value = serde_json::from_str(file_lines.as_str()).ok()?;
                    let parsed_maps = parsed.as_object()?;

                    conv_cats(parsed_maps).ok()?
                }
            ))
        }
        None => None,
    }
}

fn load_all_locales(basedir: &Path) -> HashMap<String, LocaleCategories> {
    let mut locales = HashMap::new();
    match fs::read_dir(basedir) {
        Ok(entries) => {
            for c_entry in entries {
                if let Ok(c_entry) = c_entry {
                    match load_locale(c_entry) {
                        Some((k, v)) => { locales.insert(k, v); },
                        None => (),
                    }
                }
            }
        }
        Err(e) => error!("Error while reading locale dir: {}", e),
    }
    locales
}

lazy_static! {
    static ref LOCALES: HashMap<String, LocaleCategories> = load_all_locales(&Path::new("locale"));
    static ref DEFAULT_LOCALE: String = "en_US".to_owned();
    static ref LOCALE: RwLock<String> = RwLock::new("en_US".to_owned());
}



pub fn localized(s: impl Into<&'static str>) -> &'static str {
    let s: &str = s.into();
    let s_owned = s.to_owned();
    let split: Vec<&str> = s_owned.split(':').collect();

    info!("Locale: \"{}\"", LOCALE.read().unwrap());
    info!("Default: \"{}\"", *DEFAULT_LOCALE);
    info!("all Locales: {:?}", *LOCALES);

    if split.len() != 2 {
        warn!("String \"{}\" not valid (Usage: <category>:<key>)", s);
        s
    } else {
        let locale = LOCALE.read().unwrap().clone();
        let default = DEFAULT_LOCALE.clone();
        let cat = *split.get(0).unwrap();
        let key = *split.get(1).unwrap();


        let get_key = move || {
            Some(LOCALES.get(&locale).or_else(|| LOCALES.get(&default))?
                .get(cat)?
                .get(key)?
                .as_str())
        };

        match get_key() {
            Some(st) => st,
            None => {
                warn!("Key \"{}\'not valid, using it instead of localized", s);
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringval() {
        let json: serde_json::Value = serde_json::from_str(r#"
        {
            "name": "John Doe",
            "age": 43,
            "creative": {
                "this-test": "Wow, this is a test!",
                "test2": "And another test!"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "booltest": true
        }
        "#).unwrap();

        let mut inner = LocaleStringMap::new();
        inner.insert(String::from("this-test"), String::from("Wow, this is a test!"));
        inner.insert(String::from("test2"), String::from("And another test!"));
        let mut comp = LocaleCategories::new();
        comp.insert(String::from("creative"), inner);
        let cats = conv_cats(json.as_object().unwrap()).unwrap();

        assert_eq!(cats, comp);

        let s = StringVal::from_json_value(&json).unwrap();
        println!("{}", s);
    }
}
