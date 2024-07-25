use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use futures::future::join_all;
use iced::widget::{image, Image};
use iced::widget::image::Handle;
use tokio::{fs, task};
use crate::{AppError, AppResult};
use crate::static_object::champion::{Champion, Champions};
use crate::static_object::profile_icon::{ProfileIcon, ProfileIcons};
use crate::static_object::summoner_spell::{SummonerSpell, SummonerSpells};

#[derive(Debug, Clone, Default)]
pub struct Assets {
    pub champions: HashMap<String, (Champion, Handle)>,
    pub profile_icons: HashMap<i32, Handle>,
    pub summoner_spells: HashMap<String, (SummonerSpell, Handle)>,
}

impl Assets{
    pub fn get_champion(&self, id: &str) -> Option<Handle> {
        self.champions.get(id).map(|(_, handle)| handle.clone())
    }

    pub fn get_profile_icon(&self, id: i32) -> Option<Handle> {
        self.profile_icons.get(&id).cloned()
    }

    pub fn get_summoner_spell(&self, id: &str) -> Option<Handle> {
        self.summoner_spells.get(id).map(|(_, handle)| handle.clone())
    }
}



pub async fn load_assets() -> AppResult<Assets> {
    let versions = reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json").await?.json::<Vec<String>>().await?;
    let last_version = versions.first().unwrap();
    println!("Current version: {}", last_version);
    let champions = reqwest::get(&format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json", last_version)).await?.json::<Champions>().await?;
    let profile_icons = reqwest::get(&format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/profileicon.json", last_version)).await?.json::<ProfileIcons>().await.unwrap();
    let summoner_spells = reqwest::get(&format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/summoner.json", last_version)).await?.json::<SummonerSpells>().await?;
    let assets_path = std::path::Path::new("assets");
    if !assets_path.exists(){
        std::fs::create_dir(assets_path)?;
    }

    let mut download_tasks = Vec::new();
    let mut champion_assets = HashMap::new();
    let champions_path = assets_path.join("champions");
    if !champions_path.exists(){
        fs::create_dir(&champions_path).await?;
    }
    for (_, champion) in &champions.data {
        let champion_path = champions_path.join(&champion.image.full);
        if !champion_path.exists() {
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}", last_version, &champion.image.full);
            download_tasks.push((url, champion_path));
        }
    }

    let mut profile_icon_assets = HashMap::new();
    let profile_icons_path = assets_path.join("profile_icons");
    if !profile_icons_path.exists(){
        fs::create_dir(&profile_icons_path).await?;
    }
    for (_, profile_icon) in &profile_icons.data {
        let profile_icon_path = profile_icons_path.join(&profile_icon.image.full);
        if !profile_icon_path.exists() {
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/img/profileicon/{}", last_version, profile_icon.image.full);
            download_tasks.push((url, profile_icon_path));
        }
    }

    let mut summoner_spell_assets = HashMap::new();
    let summoner_spells_path = assets_path.join("summoner_spell");
    if !summoner_spells_path.exists(){
        fs::create_dir(&summoner_spells_path).await?;
    }
    for (_, summoner_spell) in &summoner_spells.data {
        let summoner_spell_path = summoner_spells_path.join(&summoner_spell.image.full);
        if !summoner_spell_path.exists() {
            let url = format!("https://ddragon.leagueoflegends.com/cdn/{}/img/spell/{}", last_version, &summoner_spell.image.full);
            download_tasks.push((url, summoner_spell_path));
        }
    }

    let download_futures: Vec<_> = download_tasks.into_iter().map(|(url, path)| {
        task::spawn(async move {
            if !path.exists() {
                let bytes = reqwest::get(&url).await?.bytes().await?;
                fs::create_dir_all(path.parent().unwrap()).await?;
                fs::write(&path, bytes).await?;
            }
            Ok::<(),AppError>(())
        })
    }).collect();

    println!("Downloading {} assets...", download_futures.len());

    let results = join_all(download_futures).await;
    println!("Download done");
    for (_, champion) in champions.data {
        let path = champions_path.join(&champion.image.full);
        champion_assets.insert(champion.id.clone(), (champion, Handle::from_path(&path)));
    }

    for (_, profile_icon) in profile_icons.data {
        let path = profile_icons_path.join(&profile_icon.image.full);
        profile_icon_assets.insert(profile_icon.id as i32, Handle::from_path(&path));
    }

    for (_, summoner_spell) in summoner_spells.data {
        let path = summoner_spells_path.join(&summoner_spell.image.full);
        summoner_spell_assets.insert(summoner_spell.id.clone(), (summoner_spell, Handle::from_path(&path)));
    }

    Ok(Assets {
        champions: champion_assets,
        profile_icons: profile_icon_assets,
        summoner_spells: summoner_spell_assets,
    })
}