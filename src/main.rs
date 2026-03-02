use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::collections::HashSet;
use clap::Parser;
use genshin_inputmethod_dictionary::{Args, Commands};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arc::new(Args::parse());

    let (genshin_data, genshin_dictionary, language, output) = match &args.command {
        Commands::Generate { genshin_data, genshin_dictionary, output } => (genshin_data.clone(), genshin_dictionary.clone(), args.language, output.clone()),
        _ => return Err(anyhow::anyhow!("Unknown command"))
    };
    
    let root: Arc<Path> = Arc::from(genshin_data.join("src").join("data").join(language.to_dir()));

    let mut handlers = JoinSet::new();
    handlers.spawn(genshin_inputmethod_dictionary::process_domain_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_achievement_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_artifact_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_character_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_food_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_common_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_local_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_elemental_stone_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_jewel_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_talent_upgrade_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_enhancement_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_fish_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_bait_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_fishing_rod_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_furnishing_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_geography_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_monster_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_potion_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_primary_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_weapon_secondary_material_names(root.clone()));
    handlers.spawn(genshin_inputmethod_dictionary::process_dictionary(Arc::from(genshin_dictionary), language));

    let mut set = HashSet::new();
    while let Some(res) = handlers.join_next().await {
        let res = res??;

        println!("Dumped {} words", res.len());
        set.extend(res);
    }

    println!("Successfully dumped {} words", set.len());

    let dict = output
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
