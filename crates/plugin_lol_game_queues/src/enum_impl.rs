use std::fmt;
use crate::{LolGameQueuesQueue, LolGameQueuesQueueCustomGameSubcategory, LolGameQueuesQueueGameTypeConfig};

impl PartialEq for LolGameQueuesQueue{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl fmt::Display for LolGameQueuesQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl PartialEq for LolGameQueuesQueueCustomGameSubcategory{
    fn eq(&self, other: &Self) -> bool {
        self.map_id == other.map_id
    }
}


impl fmt::Display for LolGameQueuesQueueCustomGameSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.game_mode)
    }
}


impl PartialEq for LolGameQueuesQueueGameTypeConfig{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl fmt::Display for LolGameQueuesQueueGameTypeConfig{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
