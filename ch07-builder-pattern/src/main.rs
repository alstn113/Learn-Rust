#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u8,
    weight: u8,
    lifestate: LifeState,
}
#[derive(Debug)]
struct CharacterBuilder {
    name: String,
    age: u8,
    height: u8,
    weight: u8,
    lifestate: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
}

impl Default for CharacterBuilder {
    fn default() -> Self {
        Self {
            name: "John Doe".to_string(),
            age: 15,
            height: 180,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}

impl CharacterBuilder {
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }
    fn with_height(mut self, height: u8) -> Self {
        self.height = height;
        self
    }
    fn with_weight(mut self, weight: u8) -> Self {
        self.weight = weight;
        self
    }
    fn build(mut self) -> Result<Character, String> {
        if self.age < 100 && self.height < 200 && self.weight < 200 {
            Ok(Character {
                name: self.name,
                age: self.age,
                height: self.height,
                weight: self.weight,
                lifestate: self.lifestate,
            })
        } else {
            Err("Character is not fully defined".to_string())
        }
    }
}

fn main() {
    let character = CharacterBuilder::default()
        .with_name("John Doe")
        .with_age(30)
        .with_height(180)
        .with_weight(80);
    println!("{:?}", character);
    let character = character.build();
    println!("{:?}", character);
}
