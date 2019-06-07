#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Place {
    // branches
    Dungeon(u32),
    Temple,
    Lair(u32),
    Swamp(u32),
    Shoals(u32),
    Snake(u32),
    Slime(u32),
    Orc(u32),
    Elf(u32),
    Vaults(u32),
    Crypt(u32),
    Tomb(u32),
    Depths(u32),
    Hell,
    Cocytus(u32),
    Gehenna(u32),
    Tartarus(u32),
    Dis(u32),
    Abyss(u32),
    Pandemonium(u32),
    Zot(u32),

    // portals
    Bailey,
    Bazaar,
    Desolation,
    Ice,
    Gauntlet,
    Ossuary,
    Sewer,
    Treasure,
    Volcano,
    Wizlab,
    Ziggurat(u32),
}
