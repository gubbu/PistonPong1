extern crate piston_window;

pub trait Gametrait{
    fn onstart(&mut self);
    fn update(&mut self, dt: f64);
    fn render(&self, graphics: &mut piston_window::G2d, transform: [[f64; 3]; 2]);
    fn shouldquit(&self)->bool;
    fn onquit(&mut self);
    fn keyboard(&mut self, ispressed: bool, keychar: char);
}

pub fn makegame<T: Gametrait>(title: &str, size: [u32;2], fps: u64, ups: u64, gametrait: &mut T){
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new(title, size).build().unwrap();
    {
        use crate::gamewindow::piston_window::EventLoop;
        window.set_max_fps(fps);
        window.set_ups(ups);
    }
    while let Some(event) = window.next(){
        window.draw_2d(&event, |c,g,_|{gametrait.render(g,c.transform);});
        if let piston_window::Event::Loop(piston_window::Loop::Update(updateargs)) = event{
            gametrait.update(updateargs.dt);
        }
        match event{
            piston_window::Event::Input(inputenum, _) => {
                match inputenum{
                    piston_window::Input::Button(buttargs) =>{
                        if let piston_window::Button::Keyboard(key) = buttargs.button{
                            gametrait.keyboard(piston_window::ButtonState::Press == buttargs.state, pistonkey2char(key));
                        } 
                    },
                    _ =>{},
                }
            },
            _ =>{},
        } 
    }
}

fn pistonkey2char(key: piston_window::keyboard::Key)->char{
    match key{
        //generated with follwing python code:
        //>>>for i in range(ord('A'), ord('Z')+1):
        //...   print("piston_window::keyboard::Key::%c=>{return '%c'}"%(chr(i), chr(i))),
        piston_window::keyboard::Key::A=>{return 'A'},
        piston_window::keyboard::Key::B=>{return 'B'},
        piston_window::keyboard::Key::C=>{return 'C'},
        piston_window::keyboard::Key::D=>{return 'D'},
        piston_window::keyboard::Key::E=>{return 'E'},
        piston_window::keyboard::Key::F=>{return 'F'},
        piston_window::keyboard::Key::G=>{return 'G'},
        piston_window::keyboard::Key::H=>{return 'H'},
        piston_window::keyboard::Key::I=>{return 'I'},
        piston_window::keyboard::Key::J=>{return 'J'},
        piston_window::keyboard::Key::K=>{return 'K'},
        piston_window::keyboard::Key::L=>{return 'L'},
        piston_window::keyboard::Key::M=>{return 'M'},
        piston_window::keyboard::Key::N=>{return 'N'},
        piston_window::keyboard::Key::O=>{return 'O'},
        piston_window::keyboard::Key::P=>{return 'P'},
        piston_window::keyboard::Key::Q=>{return 'Q'},
        piston_window::keyboard::Key::R=>{return 'R'},
        piston_window::keyboard::Key::S=>{return 'S'},
        piston_window::keyboard::Key::T=>{return 'T'},
        piston_window::keyboard::Key::U=>{return 'U'},
        piston_window::keyboard::Key::V=>{return 'V'},
        piston_window::keyboard::Key::W=>{return 'W'},
        piston_window::keyboard::Key::X=>{return 'X'},
        piston_window::keyboard::Key::Y=>{return 'Y'},
        piston_window::keyboard::Key::Z=>{return 'Z'},
//>>> for i in range(ord('0'), ord('9')+1):
//...     print("piston_window::keyboard::Key::D%c=>{return '%c'},"%(chr(i), chr(i)))
        piston_window::keyboard::Key::D0=>{return '0'},
        piston_window::keyboard::Key::D1=>{return '1'},
        piston_window::keyboard::Key::D2=>{return '2'},
        piston_window::keyboard::Key::D3=>{return '3'},
        piston_window::keyboard::Key::D4=>{return '4'},
        piston_window::keyboard::Key::D5=>{return '5'},
        piston_window::keyboard::Key::D6=>{return '6'},
        piston_window::keyboard::Key::D7=>{return '7'},
        piston_window::keyboard::Key::D8=>{return '8'},
        piston_window::keyboard::Key::D9=>{return '9'},
        piston_window::keyboard::Key::Space=>{return ' '},
        piston_window::keyboard::Key::Return=>{return '\r'},
        piston_window::keyboard::Key::Tab=>{return '\t'},
        piston_window::keyboard::Key::Period=>{return '.'},
        piston_window::keyboard::Key::Question=>{return '?'},
        piston_window::keyboard::Key::Exclaim=>{return '!'},
        _ => {return '弈';},
    }
}