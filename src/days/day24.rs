extern crate regex;

use std::error::Error;

use std::fs;
use std::path::Path;

use std::cmp;

use self::regex::Regex;

#[derive(Clone, Debug)]
struct Group {
    units: usize,
    unit_hp: usize,
    weaknesses: Option<String>,
    immunities: Option<String>,
    unit_power: usize,
    atk_type: String,
    initiative: usize,
    faction: usize,
    targeting: Option<usize>,
    targeted: bool
}

impl Group {
    fn total_power(&self) -> usize {
        self.units * self.unit_power
    }

    fn inflicted_damage(&self, damage: usize, atk_type: &String) -> usize {
        let mut real_damage = damage;

        if let Some(ref weakness_list) = self.weaknesses {
            if weakness_list.contains(atk_type) {
                real_damage *= 2;
            }
        }

        if let Some(ref immunity_list) = self.immunities {
            if immunity_list.contains(atk_type) {
                real_damage = 0;
            }
        }

        real_damage
    }

    fn receive_damage(&mut self, damage: usize) -> bool {
        let lost_units = damage / self.unit_hp;
        self.units -= cmp::min(self.units, lost_units);
        self.units == 0
    }
}

fn prepare_input() -> Vec<Group> {
    let input = fs::read_to_string(Path::new("./data/day24.txt")).unwrap();
    let splitted: Vec<&str> = input.split("Infection:").collect();
    let unit_reg = Regex::new(r"(\d+).*?(\d+).*? points (\(.*\))?.*?(\d+) (\w+).*?(\d+)").unwrap();
    let weaknesses_reg = Regex::new(r"weak to (.*?)[;|\)]").unwrap();
    let immunities_reg = Regex::new(r"immune to (.*?)[;|\)]").unwrap();

    let mut armies: Vec<Group> = Vec::new();

    for (index, army) in splitted.iter().enumerate() {
        for cap in unit_reg.captures_iter(army) {
            let specials = cap.get(3).map_or("", |m| m.as_str());
            let mut weaknesses = None;
            let mut immunities = None;

            if let Some(weak_list) = weaknesses_reg.captures(&specials) {
                weaknesses = Some(weak_list[1].to_string());
            }

            if let Some(immune_list) = immunities_reg.captures(&specials) {
                immunities = Some(immune_list[1].to_string());
            }

            armies.push(Group{
                units: cap[1].parse().unwrap(),
                unit_hp: cap[2].parse().unwrap(),
                immunities,
                weaknesses,
                unit_power: cap[4].parse().unwrap(),
                atk_type: cap[5].to_string(),
                initiative: cap[6].parse().unwrap(),
                faction: index,
                targeting: None,
                targeted: false
            });
        }
    }

    armies
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut armies = prepare_input();

    while armies.iter().filter(|x| x.faction == 0).count() > 0 && armies.iter().filter(|x| x.faction == 1).count() > 0 {
        let mut armies_selection: Vec<usize> = (0..armies.len()).collect();
        let mut armies_attack: Vec<usize> = Vec::new();
        {
            armies_selection.sort_by(|&unit_index_a, &unit_index_b| {
                let unit_a = &armies[unit_index_a];
                let unit_b = &armies[unit_index_b];
                let pow_a = unit_a.total_power();
                let pow_b = unit_b.total_power();

                if pow_b == pow_a {
                    unit_a.initiative.cmp(&unit_b.initiative)
                } else {
                    pow_a.cmp(&pow_b)
                }
            });
        }

        // Selection
        while !armies_selection.is_empty() {
            let group_id = armies_selection.pop().unwrap();
            let total_power;
            let atk_type;
            let faction;

            let mut target_index = None;
            {
                let group = &armies[group_id];
                total_power = group.total_power();
                atk_type = group.atk_type.clone();
                faction = group.faction;
            }

            if let Some((index, target)) = armies.iter_mut().enumerate().filter(|(_, target)| target.faction != faction && !target.targeted && target.inflicted_damage(total_power, &atk_type) != 0).max_by(|(_, t_a), (_, t_b)| {
                let t_inflicted_a = t_a.inflicted_damage(total_power, &atk_type);
                let t_inflicted_b = t_b.inflicted_damage(total_power, &atk_type);
                let t_effective_p_a = t_a.total_power();
                let t_effective_p_b = t_b.total_power();

                if t_inflicted_a != t_inflicted_b {
                    t_inflicted_a.cmp(&t_inflicted_b)
                } else if t_effective_p_a != t_effective_p_b {
                    t_effective_p_a.cmp(&t_effective_p_b)
                } else {
                    t_a.initiative.cmp(&t_b.initiative)
                }
            }) {
                target.targeted = true;
                target_index = Some(index);
            }

            let group = armies.get_mut(group_id).unwrap();
            group.targeting = target_index;

            armies_attack.push(group_id);
        }

        {
            armies_attack.sort_by(|&unit_index_a, &unit_index_b| {
                let unit_a = &armies[unit_index_a];
                let unit_b = &armies[unit_index_b];
                unit_a.initiative.cmp(&unit_b.initiative)
            });
        }

        // ATTACC
        while !armies_attack.is_empty() {
            let attacker_id = armies_attack.pop().unwrap();

            let mut total_power;
            let target_id;
            let atk_type;

            {
                let attacker = armies.get_mut(attacker_id).unwrap();

                if attacker.units == 0 || attacker.targeting.is_none() {
                    attacker.targeting = None;
                    continue;
                }

                total_power = attacker.total_power();
                target_id = attacker.targeting.unwrap();
                atk_type = attacker.atk_type.clone();
                attacker.targeting = None;
            }

            let target = armies.get_mut(target_id).unwrap();

            total_power = target.inflicted_damage(total_power, &atk_type);

            target.targeted = false;
            target.receive_damage(total_power);
        }

        armies = armies.iter().cloned().filter(|x| x.units > 0).collect();
    }

    let answer = armies.iter().fold(0, |acc, group| {
        acc + group.units
    });

    println!("Winning army end up with {} units", answer);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}