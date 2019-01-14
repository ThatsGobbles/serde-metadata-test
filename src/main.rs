extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Deserialize)]
#[serde(untagged)]
pub enum MetaVal {
    Nil,
    Str(String),
    Seq(Vec<MetaVal>),
    Map(BTreeMap<String, MetaVal>),
}

type MetaBlock = BTreeMap<String, MetaVal>;
type MetaBlockSeq = Vec<MetaBlock>;
type MetaBlockMap = HashMap<String, MetaBlock>;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MetaStructureRepr {
    Unit(UnitMetaStructureRepr),
    Many(ManyMetaStructureRepr),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum UnitMetaStructureRepr {
    One(MetaBlock),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ManyMetaStructureRepr {
    Seq(MetaBlockSeq),
    Map(MetaBlockMap),
}

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Unit,
    Many,
}

pub type FallbackSpec = HashMap<String, FallbackSpecNode>;

/// Node type for the tree representation of fallback methods.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FallbackSpecNode {
    Leaf(FallbackMethod),
    Pass(HashMap<String, FallbackSpecNode>),
    Both(FallbackMethod, HashMap<String, FallbackSpecNode>),
}

/// Different ways to process parent metadata into desired outputs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackMethod {
    Inherit,
    Collect,
}


fn handle(fp: &str, target: Target) -> MetaStructureRepr {
    let mut f = File::open(fp).expect("cannot open file");

    let mut text = String::new();
    f.read_to_string(&mut text).expect("cannot read file");

    match target {
        Target::Unit => MetaStructureRepr::Unit(serde_json::from_str(&text).unwrap()),
        Target::Many => MetaStructureRepr::Many(serde_json::from_str(&text).unwrap()),
    }
}

fn main() {
    // let mut f = File::open("self.json").expect("cannot open file");
    // let mut f = File::open("item.json").expect("cannot open file");

    let parsed = handle("self.json", Target::Unit);
    println!("{:#?}", parsed);

    let parsed = handle("item.json", Target::Many);
    println!("{:#?}", parsed);

    let parsed = handle("item_map.json", Target::Many);
    println!("{:#?}", parsed);

    let fallback_text = r#"
        {
            "title": "inherit",
            "rg": [
                "inherit",
                {
                    "gain": "collect"
                }
            ],
            "other": {
                "sub_a": "collect",
                "sub_b": {
                    "sub_sub_a": "inherit",
                    "sub_sub_b": "collect"
                }
            }
        }
    "#;

    let fb_spec: FallbackSpec = serde_json::from_str(&fallback_text).unwrap();
    println!("{:#?}", fb_spec);

    // // scope to control lifetime of borrow
    // {
    //     // Extract the rate
    //     let rate = sample.get("rate").unwrap().as_u64().unwrap();
    //     println!("rate: {}", rate);

    //     // Extract the array
    //     let array : &mut Vec<Value> = sample.get_mut("array").unwrap().as_array_mut().unwrap();
    //     println!("first: {}", array.get(0).unwrap());

    //     // Add a value
    //     array.push(Value::String("tak".to_string()));
    // }

    // // Encode to Hjson
    // let sample2 = serde_hjson::to_string(&sample).unwrap();
    // println!("Hjson:\n{}", sample2);
}
