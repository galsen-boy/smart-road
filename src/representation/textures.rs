use macroquad::prelude::*;

const CAR_SPRITES: [&str; 9] = [
    "assets/cars/1.png",
    "assets/cars/2.png",
    "assets/cars/6.png",
     "assets/cars/4.png",
    "assets/cars/5.png",
    "assets/cars/3.png",
    "assets/cars/7.png",
    "assets/cars/ambulance.png",
    "assets/cars/garbage_truck.png",
    
];

#[derive(PartialEq, Clone)]
pub struct Textures {
    pub bg: Texture2D,
    pub cars: Vec<Texture2D>,
}

impl Textures {
    pub async fn load() -> Self {
        let mut cars = Vec::new();
        for sprite in CAR_SPRITES {
            cars.push(load_texture(sprite).await.unwrap())
        }
        Self {
            bg: load_texture("assets/road.png").await.unwrap(),
            cars,
        }
    }
}
