use console::Term;

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
}

struct Car {
    
}

enum Pieces {
    Car(Car),
    Road(char),
}

struct Game {
    area: Vec<Vec<Pieces>>
}

enum State {
    Play,
    Menu,
    ChangeKeybindings,
    Exit
}

fn main() {

}