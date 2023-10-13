fn rand_summon_entity( entity: u8) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let rand_pos = Position {
        x: rng.gen_range(0..self.width),
        y: rng.gen_range(0..self.height)
    };
    self.summon_entity(entity, rand_pos)
}