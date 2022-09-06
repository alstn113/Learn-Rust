use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {}

#[derive(Debug)]
struct Elf {}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl Magic for Wizard {}
impl FightClose for Wizard {}
impl FightClose for Elf {}
impl FightFromDistance for Elf {}

fn attack_with_bow<T>(character: &T, oppenent: &mut Monster, distance: u32)
where
    T: FightFromDistance + Debug,
{
    if distance < 10 {
        oppenent.health -= 10;
        println!(
            "Monster's health : {}, character's health : {:?}",
            oppenent.health, character
        );
    }
}

fn attack_with_sword<T>(character: &T, oppenent: &mut Monster)
where
    T: FightClose + Debug,
{
    oppenent.health -= 20;
    println!(
        "Monster's health : {}, character : {:?}",
        oppenent.health, character
    );
}

fn attack_with_fireball<T>(character: &T, oppenent: &mut Monster, distance: u32)
where
    T: Magic + Debug,
{
    if distance < 15 {
        oppenent.health -= 30;
        println!(
            "Monster's health : {}, character : {:?}",
            oppenent.health, character
        );
    }
}

fn main() {
    let wizard = Wizard {};
    let elf = Elf {};

    let mut monster = Monster { health: 100 };

    attack_with_bow(&elf, &mut monster, 5);
    attack_with_fireball(&wizard, &mut monster, 10);
    attack_with_sword(&wizard, &mut monster);
}
