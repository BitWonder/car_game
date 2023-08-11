use std::{io::{self, Write}, thread, time::Duration};
// for keyboard
use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(Copy, Clone)]
struct Keybindings {
    forward: char,
    backward: char,
    left: char,
    right: char,
    nitro: char,
    enter: char,
    pause: char,
}

impl Keybindings {
    fn default() -> Keybindings {
        Keybindings { forward: 'w', backward: 's', left: 'a', right: 'd', nitro: 'x', enter: '\n', pause: 'q' }
    }

    fn change(mut self, place: u8, key: char) {
        match place {
            1 => self.forward = key,
            2 => self.backward = key,
            3 => self.left = key,
            4 => self.right = key,
            5 => self.nitro = key,
            6 => self.enter = key,
            7 => self.pause = key,
            _ => self.forward = self.forward
        }
    }

    fn key_bind_change(&mut self) {
        *self;
    }
}

struct Car {
    model: String,
    speed: u8,
    lane: u8,
    acceleration: u8,
}

impl Car {
    fn new() -> Car {
        return Car { model : "()".to_string(), speed : 0, lane : 0, acceleration : 0 };
    }
}

enum Pieces {
    Car(Car),
    Road(char),
}

struct Game {
    area: Vec<Vec<Pieces>>,
    car: Car,
    need_new_game: bool
}

impl Game {
    fn setup() -> Game {
        Game { area: 8, car: Car::new(), need_new_game: true }
    }

    fn play(&mut self) {

    }
}

enum State {
    Play,
    Menu,
    ChangeKeybindings,
    Exit
}

fn clear_screen() {
    // if windows or unix then clear screen
    if cfg!(target_os = "windows") {
        // clears the windows screen
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        // clears the unix screen
        std::process::Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    }
}

fn menuing() {

}

fn keys_down() -> Vec<Keycode> {
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();
    return keys;
}

fn main() {
    let mut key_bin = Keybindings::default();
    // start game in menu
    let mut game_state = State::Menu;
    //ready play
    let mut game = Game::setup();

    'game_loop : loop {
        match game_state {
            State::Menu => menuing(),
            State::ChangeKeybindings => key_bin.key_bind_change(),
            State::Exit => break 'game_loop,
            _ => game.play()
        }
    }
}