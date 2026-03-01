mod model;

use std::io::BufReader;
use std::collections::HashSet;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs::File;
use regex::Regex;
use clap::{Parser, Subcommand, ValueEnum};
use model::*;

#[derive(Clone, Copy, ValueEnum)]
enum Language {
    /// Simplified Chiense
    ZhHans,
    /// Traditional Chinese
    ZhHant
}

impl Language {
    fn to_dir(&self) -> &'static str {
        match self {
            Self::ZhHans => "chinese-simplified",
            Self::ZhHant => "chinese-traditional"
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Generate dictionary
    Generate
}

#[derive(Parser)]
#[command(
    name = "GenshinInputmethodDictionary",
    version,
    author,
    about,
    propagate_version = true
)]
pub struct Args {
    /// Path to https://github.com/dvaJi/genshin-data directory
    #[arg(short, long)]
    data: PathBuf,
    /// Path to output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// Language variant
    #[arg(short, long, value_enum, default_value_t = Language::ZhHans)]
    language: Language,
    #[command(subcommand)]
    command: Commands
}

pub async fn process_domain_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Domains> {
        let file = File::open(root.join("domains.json"))?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        
        Ok(data)
    }).await??;

    let pat = Regex::new(r"精通秘境：(?<name>.+) IV")?;
    for d in data.characters {
        let caps = pat.captures(&d.domain_name).unwrap();
        set.insert(Box::from(&caps["name"]));
    }
    let pat = Regex::new(r"炼武秘境：(?<name>.+)")?;
    for d in data.weapons {
        if let Some(caps) = pat.captures(&d.domain_name) {
            set.insert(Box::from(&caps["name"]));
        }
        else {
            set.insert(d.domain_name);
        }
    }

    Ok(set)
}

pub async fn process_food_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("food").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Food> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
        if let Some(special) = data.results.special {
            set.insert(special.name);
        }
    }

    Ok(set)
}

pub async fn process_common_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("common_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<CommonMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_local_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("local_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<LocalMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_elemental_stone_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("elemental_stone_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<ElementalStoneMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_jewel_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("jewels_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<JewelMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_talent_upgrade_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("talent_lvl_up_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<TalentLvlUpMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_weapon_enhancement_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("weapon_enhancement_material").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<WeaponEnhancementMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_character_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("characters").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Character> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
        set.insert(data.affiliation);
        set.insert(data.constellation);
        if let Some(title) = data.title {
            set.insert(title);
        }
        set.insert(data.element.name);
        set.insert(data.region.name);
        for c in data.constellations {
            set.insert(c.name);
        }
        for s in data.skills {
            set.insert(s.name);
        }
        for p in data.passives {
            set.insert(p.name);
        }
    }

    Ok(set)
}

pub async fn process_weapon_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("weapons").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Weapon> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_achievement_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("achievements").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Achievements> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
        for a in data.achievements {
            set.insert(a.name);
        }
    }

    Ok(set)
}

pub async fn process_artifact_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("artifacts").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Artifacts> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
        set.insert(data.circlet.name);
        if let Some(g) = data.goblet {
            set.insert(g.name);
        }
        if let Some(p) = data.plume {
            set.insert(p.name);
        }
        if let Some(f) = data.flower {
            set.insert(f.name);
        }
        if let Some(s) = data.sands {
            set.insert(s.name);
        }
    }

    Ok(set)
}

pub async fn process_fish_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("fish").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Fish> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_bait_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("bait").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Bait> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_fishing_rod_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("fishing_rod").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<FishingRod> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_furnishing_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("furnishing").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Furnishing> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_geography_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("geography").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Geography> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
        if let Some(a) = data.area_name {
            set.insert(a);
        }
    }

    Ok(set)
}

pub async fn process_monster_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("monsters").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Monster> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
        for s in data.special_names {
            if let Some(s) = s {
                set.insert(s);
            }
        }
    }

    Ok(set)
}

pub async fn process_potion_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("potions").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<Potion> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_weapon_primary_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("weapon_primary_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<WeaponPrimaryMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}

pub async fn process_weapon_secondary_material_names(args: Arc<Args>) -> anyhow::Result<HashSet<Box<str>>> {
    let mut set = HashSet::new();
    let root = args.data.join("src").join("data").join(args.language.to_dir());

    for e in root.join("weapon_secondary_materials").read_dir()? {
        let path = e?.path();
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<WeaponSecondaryMaterial> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            
            Ok(data)
        }).await??;

        set.insert(data.name);
    }

    Ok(set)
}
