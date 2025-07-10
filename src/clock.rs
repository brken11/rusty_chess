use std::time::{Duration, Instant};
use crate::board::Color;
use crate::rules::{TimeControls, Timer};

pub struct ChessClock{
    running: bool,
    active_player: Color,
    white_total_time: Duration,
    white_time_left: Duration,
    black_total_time: Duration,
    black_time_left: Duration,
    instant: Instant,
}

impl ChessClock{
    fn new() -> Self{
        Self{
            running: false,
            active_player: Color::White,
            white_total_time: Duration::from_secs(0),
            white_time_left: Duration::from_secs(0),
            black_total_time: Duration::from_secs(0),
            black_time_left: Duration::from_secs(0),
            instant: Instant::now()
        }
    }
    
}

impl Timer for ChessClock{
    fn new_from_time_controls(time_controls: TimeControls) -> Option<Self> {
        let white_time = time_controls.initial_time_white;
        let black_time = time_controls.initial_time_black;
        if white_time.as_secs() < 0 || black_time.as_secs() < 0 {
            None
        } else {
            Some(Self{
                running: false,
                active_player: Color::White,
                white_total_time: white_time,
                white_time_left: white_time,
                black_total_time: black_time,
                black_time_left: black_time,
                instant: Instant::now()
            })
        }
    }
    
    fn start(&mut self) {
        if !self.running{
            self.instant = Instant::now();
        }
        self.running = true;
    }

    fn stop(&mut self) {
        if self.running{
            match self.active_player {
                Color::White => self.white_time_left -= self.instant.elapsed(),
                Color::Black => self.black_time_left -= self.instant.elapsed(),
            }
        }
        self.running = false;       
    }

    fn switch_clock(&mut self) {
        if self.running {
            match self.active_player {
                Color::White => self.white_time_left -= self.instant.elapsed(),
                Color::Black => self.black_time_left -= self.instant.elapsed(),
            }
            self.instant = Instant::now();
        }
        
        self.active_player = self.active_player.toggle_color();
    }

    fn reset(&mut self) {
        self.running = false;
        self.white_time_left = self.white_total_time;
        self.black_time_left = self.black_total_time;
        self.active_player = Color::White;
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn active_player(&self) -> Color {
        self.active_player
    }

    fn get_total_time(&self) -> Duration {
        self.white_total_time + self.black_total_time
    }
    
    fn get_time_left(&self) -> Duration {
        self.white_time_left + self.black_time_left
    }

    fn get_active_time_left(&self) -> Duration {
        self.get_player_time_left(self.active_player)
    }

    fn get_player_time_left(&self, color: Color) -> Duration {
        let time_left = match color {
            Color::White => self.white_time_left,
            Color::Black => self.black_time_left,
        };
        
        if self.running && self.active_player == color{
            time_left - self.instant.elapsed()
        }else{
            time_left
        }
    }

    fn set_total_time(&mut self, time: Duration) {
        self.white_total_time= time;
        self.black_total_time= time;
    }

    fn set_player_time_left(&mut self, color: Color, time: Duration) {
        match color {
            Color::White => self.white_time_left = time,
            Color::Black => self.black_time_left = time
        }
    }
}