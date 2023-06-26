pub mod player {
  pub struct Player {
    pub nickname: String,
    pub score: i32
  }

  impl Player {
      pub fn new(nickname: String, score: i32) -> Self 
      {
        Self {nickname, score}
      }

      pub fn increment_score(&mut self, value: i32) -> i32
      {
        self.score = self.score + value;
        return self.score;
      }

      pub fn decrement_score(&mut self, value: i32) -> i32
      {
        self.score = self.score - value;
        return self.score;
      }

      pub fn get_score(&self) -> i32 
      {
        return self.score;
      }
      pub fn get_nickname(&self) -> String 
      {
        return self.nickname.clone();
      }
  }
}