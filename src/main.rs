use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::collections::HashSet;
use clap::Parser;
use genshin_inputmethod_dictionary::Args;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arc::new(Args::parse());

    let mut handlers = JoinSet::new();
    handlers.spawn(genshin_inputmethod_dictionary::process_domain_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_achievement_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_artifact_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_character_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_food_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_common_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_local_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_elemental_stone_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_jewel_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_talent_upgrade_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_enhancement_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_fish_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_bait_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_fishing_rod_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_furnishing_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_geography_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_monster_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_potion_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_primary_material_names(args.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_secondary_material_names(args.clone()));

    let mut set = HashSet::new();
    while let Some(res) = handlers.join_next().await {
        let res = res??;

        println!("Dumped {} words", res.len());
        set.extend(res);
    }

    println!("Successfully dumped {} words", set.len());

    let dict = args.output
        .clone()
        .unwrap_or(PathBuf::from_str(".").unwrap())
        .join("dictionary.txt");
    let content = set
        .into_iter()
        .map(str::into_string)
        .map(Cow::from)
        .reduce(|mut acc, s| {
            acc.to_mut().push('\n');
            acc.to_mut().push_str(&s);

            acc
        })
        .unwrap();
    tokio::fs::write(&dict, content.as_bytes()).await?;
    println!("All text has been written into `{}`", dict.display());

    Ok(())
}
