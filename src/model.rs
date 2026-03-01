use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "domainName")]
    pub domain_name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Domains {
    pub characters: Vec<Domain>,
    pub weapons: Vec<Domain>
}

#[derive(Serialize, Deserialize)]
pub struct Achievement {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Achievements {
    pub name: Box<str>,
    pub achievements: Vec<Achievement>
}

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Artifacts {
    pub name: Box<str>,
    pub goblet: Option<Artifact>,
    pub plume: Option<Artifact>,
    pub circlet: Artifact,
    pub flower: Option<Artifact>,
    pub sands: Option<Artifact>
}

#[derive(Serialize, Deserialize)]
pub struct Element {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Region {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Constellation {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: Box<str>,
    pub affiliation: Box<str>,
    pub constellation: Box<str>,
    pub domain: Box<str>,
    pub title: Option<Box<str>>,
    pub element: Element,
    pub region: Region,
    pub skills: Vec<Skill>,
    pub passives: Vec<Skill>,
    pub constellations: Vec<Constellation>
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Food {
    pub name: Box<str>,
    pub results: Results
}

#[derive(Serialize, Deserialize)]
pub struct Results {
    pub suspicious: Delicious,
    pub normal: Delicious,
    pub delicious: Delicious,
    pub special: Option<Special>
}

#[derive(Serialize, Deserialize)]
pub struct Delicious {
    pub name: Box<str>,
    pub description: Box<str>,
    pub effect: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Special {
    pub name: Box<str>,
    pub character: CharacterRef
}

#[derive(Serialize, Deserialize)]
pub struct CharacterRef {
    pub id: Box<str>,
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct CommonMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct LocalMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct ElementalStoneMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct JewelMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct TalentLvlUpMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct WeaponEnhancementMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct WeaponPrimaryMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct WeaponSecondaryMaterial {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Fish {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Bait {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct FishingRod {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Furnishing {
    pub name: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct Geography {
    pub name: Box<str>,
    #[serde(rename = "areaName")]
    pub area_name: Option<Box<str>>
}

#[derive(Serialize, Deserialize)]
pub struct Monster {
    pub name: Box<str>,
    #[serde(rename = "specialNames")]
    pub special_names: Vec<Option<Box<str>>>
}

#[derive(Serialize, Deserialize)]
pub struct Potion {
    pub name: Box<str>
}
