use crate::game::{GameMessage, GameResponse, PlayerType};
use std::sync::mpsc;

mod local_handler;
mod remote_handler;

pub struct PlayerHandler{
    player_type: PlayerType,
    player_receiver: mpsc::Receiver<GameResponse>,
    player_sender: mpsc::Sender<GameMessage>,
}
 impl PlayerHandler{
     pub fn new(player_type: PlayerType, player_sender: mpsc::Sender<GameMessage>, player_receiver: mpsc::Receiver<GameResponse>) -> Self {
         PlayerHandler{
            player_type,
            player_receiver,
            player_sender,
         }
     }
 }