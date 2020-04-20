use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::collections::HashSet;

pub struct Keyboard {
    event_pump: EventPump,
    keys_pressed: HashSet<Keycode>,
}

pub struct Key(pub u8);

impl From<Keycode> for Key {
    fn from(keycode: Keycode) -> Self {
        match keycode {
            Keycode::Num1 => Key(1),
            Keycode::Num2 => Key(2),
            Keycode::Num3 => Key(3),
            Keycode::Num4 => Key(4),
            Keycode::Num5 => Key(5),
            Keycode::Num6 => Key(6),
            Keycode::Num7 => Key(7),
            Keycode::Num8 => Key(8),
            Keycode::Num9 => Key(9),
            Keycode::Num0 => Key(0),
            Keycode::A => Key(0xA),
            Keycode::B => Key(0xB),
            Keycode::C => Key(0xC),
            Keycode::D => Key(0xD),
            Keycode::E => Key(0xE),
            Keycode::F => Key(0xF),
            _ => unimplemented!(),
        }
    }
}

impl Into<Keycode> for Key {
    fn into(self) -> Keycode {
        match self {
            Key(1) => Keycode::Num1,
            Key(2) => Keycode::Num2,
            Key(3) => Keycode::Num3,
            Key(4) => Keycode::Num4,
            Key(5) => Keycode::Num5,
            Key(6) => Keycode::Num6,
            Key(7) => Keycode::Num7,
            Key(8) => Keycode::Num8,
            Key(9) => Keycode::Num9,
            Key(0) => Keycode::Num0,
            Key(0xA) => Keycode::A,
            Key(0xB) => Keycode::B,
            Key(0xC) => Keycode::C,
            Key(0xD) => Keycode::D,
            Key(0xE) => Keycode::E,
            Key(0xF) => Keycode::F,
            _ => unimplemented!(),
        }
    }
}

enum KeyEvent {
    KeyDown(Key),
    KeyUp(Key),
}

impl Keyboard {
    pub fn new(event_pump: EventPump) -> Keyboard {
        Keyboard {
            event_pump,
            keys_pressed: HashSet::new(),
        }
    }

    pub fn process_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.keys_pressed.insert(key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.keys_pressed.remove(&key);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn next_key(&mut self) -> Key {
        loop {
            let event = self.event_pump.wait_event();
            let event = self.process_event(event);
            if let Some(KeyEvent::KeyDown(key)) = event {
                return key;
            }
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key.into())
    }

    fn process_event(&mut self, event: Event) -> Option<KeyEvent> {
        match event {
            Event::KeyDown { keycode, .. } => {
                if let Some(key) = keycode {
                    let event = key.clone().into();
                    self.keys_pressed.insert(key);
                    Some(KeyEvent::KeyDown(event))
                } else {
                    None
                }
            }
            Event::KeyUp { keycode, .. } => {
                if let Some(key) = keycode {
                    let event = key.clone().into();
                    self.keys_pressed.remove(&key);
                    Some(KeyEvent::KeyUp(event))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
