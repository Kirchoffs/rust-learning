#[cfg(test)]
mod test {
    pub trait Sport {
        type SportType;
        
        fn play(&self,  st: Self::SportType);
    }
    
    struct Football;
    pub enum SportType {
      Land,
      Water,
    }
    
    impl Sport for Football {
      type SportType = SportType;
      fn play(&self,  sport_type: Self::SportType){}
    }
    
    #[test]
    fn trait_demo() {
      let f = Football;
      f.play(SportType::Land);
    }
}