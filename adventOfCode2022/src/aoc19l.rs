
 
pub type Num = u8;

#[derive(Debug,Clone)]
pub struct Cost {
    pub ore: Num,
    pub clay: Num,
    pub obsidian: Num,
}

#[derive(Debug,Clone)]
pub struct Blueprint {
    pub id: Num,
    pub ore_robot: Cost,
    pub clay_robot: Cost,
    pub obsidian_robot: Cost,
    pub geode_robot: Cost,
}
