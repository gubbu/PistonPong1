mod aabbdump;
mod gamewindow;

const GAMERECT: [f64;4] = [0.0, 0.0, 500.0, 500.0];
const PLAYERHEIGHT: f64 = GAMERECT[3]/4.0;
const PLAYER1: [f64;4] = [0.01, 0.01, PLAYERHEIGHT/20.0+0.01, PLAYERHEIGHT+0.01];
const PLAYER2: [f64;4] = [
    GAMERECT[0]+GAMERECT[2]-PLAYERHEIGHT/20.0-0.01,
    GAMERECT[1]+GAMERECT[3]-PLAYERHEIGHT-0.01,
    GAMERECT[0]+GAMERECT[2]-0.01,
    GAMERECT[1]+GAMERECT[3]-0.01];

const BALLACC: f64 = 11.0;
const ACCSELERATION: f64 = 10.0;
const PIXELSPERSECOND: f64 = 20.0;
//COLORS
const BALLCOLOR: [f32;4] = [0.0, 0.0, 1.0, 1.0];
const PLAYER1C: [f32;4] = [1.0, 0.0, 0.0, 1.0];
const PLAYER2C: [f32;4] = [1.0, 1.0, 0.0, 1.0];
const BALLSIZE: f64 = 10.0;

enum Dir{
    UP = 0,
    DOWN = 1,
    STANDING = 2,
}

//stupid random number generator
fn random(seed: f64)->f64{
    ((seed*9809.02421).sin()*9992.2342942).fract()
}

//#[derive(Debug)]
struct Player{
    rect: [f64;4],
    score: u32,
    dir: Dir,
    //acceleration timer for SMOOTH experience:
    acc_timer: f64,
}

impl Player{
    fn simulateplayer(&mut self, dts: f64){
        use aabbdump::*;
        self.acc_timer += dts;
        self.playermove(dts);
        if !aabbdump::a_allin_b(&self.rect, &GAMERECT){
            println!("OH NO {:?}", self.rect);
            self.playermove(-dts);
            self.acc_timer = 0.0;
            self.dir = Dir::STANDING;
        }
    }

    fn playermove(&mut self, dts: f64){
        use aabbdump::*;
        match &self.dir{
            Dir::UP => {
                //println!("MOVING UP");
                movrect(&0.0, &-1.0, &(dts*PIXELSPERSECOND*(self.acc_timer*ACCSELERATION+1.0)), &mut self.rect);
            },
            Dir::DOWN => {movrect(&0.0, &1.0, &(dts*PIXELSPERSECOND*(self.acc_timer*ACCSELERATION+1.0)), &mut self.rect);},
            _ => {self.acc_timer = 0.0},
        }
    }
}

struct Ponggame{
    p1: Player,
    p2: Player,
    ball: [f64;2],
    balldir: [f64;2],
    ballspeed: f64,
    //as game mechanic: the time stopped before the ball collides with something
    current_game_timer: f64,
}

impl Ponggame{
    fn newstandart()->Self{
        let mut pong = Ponggame{
            p1: Player{rect: PLAYER1, score: 0, dir: Dir::STANDING, acc_timer: 0.0},
            p2: Player{rect: PLAYER2, score: 0, dir: Dir::STANDING, acc_timer: 0.0},
            ball: [0.0;2],
            balldir: [0.0;2],
            ballspeed: 100.0,
            current_game_timer: 0.0,
        };
        pong.respawnball();
        return pong;
    }

    fn on_score(&mut self){
        self.current_game_timer = 0.0;
        println!("SCORE: P1: {} vs. P2: {}", self.p1.score, self.p2.score);
        self.respawnball();
    }

    fn wallcoll(&mut self, dts: f64){
        self.balldir[1] *= -1.0;
        aabbdump::add(&mut self.ball, &self.balldir, dts*self.ballspeed*(self.current_game_timer+1.0));
        self.current_game_timer = 0.0;        
    }

    fn simulate(&mut self, dts: f64){
        self.current_game_timer += dts;
        self.p1.simulateplayer(dts);
        self.p2.simulateplayer(dts);
        aabbdump::add(&mut self.ball, &self.balldir, dts*self.ballspeed*(self.current_game_timer+1.0));
        if self.ball[0] < GAMERECT[0]{
            //println!("P1 scored P1: {} vs. P2: {}", self.p1.score, self.p2.score);
            self.p1.score += 1;
            self.on_score();
        }else if self.ball[0] > GAMERECT[2]{
            //println!("P1 scored P2: {} vs. P2: {}", self.p1.score, self.p2.score);
            self.p2.score += 1;
            self.on_score();
        }else if self.ball[1] < GAMERECT[1] || self.ball[1] > GAMERECT[3]{
            self.wallcoll(dts);
        }else if aabbdump::rectpointcoll(&self.p1.rect, &self.ball[0], &self.ball[1]) || aabbdump::rectpointcoll(&self.p2.rect, &self.ball[0], &self.ball[1]){
            self.balldir[0] *= -1.0;
            //1. remove the movement that lead to this collision:
            aabbdump::add(&mut self.ball, &self.balldir, dts*self.ballspeed*(self.current_game_timer+1.0));
            self.current_game_timer = 0.0;
        }

    }

    fn respawnball(&mut self){
        self.ballspeed += 0.5;
        self.ball = aabbdump::middlepoint(&GAMERECT);
        self.balldir = aabbdump::fromang(random((self.p1.score/2+self.p2.score) as f64+33.0)*2.0*std::f64::consts::PI, 1.0);
    }
}

impl gamewindow::Gametrait for Ponggame{
    fn onstart(&mut self){println!("starting")}

    fn update(&mut self, dt: f64){
        //println!("delta time {} s", dt)
        self.simulate(dt)
        //self.p1.simulateplayer(dt);
        //self.p2.simulateplayer(dt);
    }

    fn render(&self, g: &mut piston_window::G2d, transform: [[f64; 3]; 2]){
        //println!("transform {:?}", transform);
        piston_window::clear([0.0, 0.0, 0.0, 0.5], g);

        piston_window::rectangle(PLAYER1C,
         [self.p1.rect[0], self.p1.rect[1], self.p1.rect[2]-self.p1.rect[0], self.p1.rect[3]-self.p1.rect[1]]
        , transform, g);

        piston_window::rectangle(PLAYER2C,
         [self.p2.rect[0], self.p2.rect[1], self.p2.rect[2]-self.p2.rect[0], self.p2.rect[3]-self.p2.rect[1]]
        , transform, g);

        //println!("BALL {:?}", self.ball);
        piston_window::ellipse(BALLCOLOR, [self.ball[0]-BALLSIZE*0.5, self.ball[1]-BALLSIZE*0.5, BALLSIZE, BALLSIZE], transform, g);
    }
    
    fn shouldquit(&self)->bool{return false;}
    fn onquit(&mut self){println!("QUITING!");}
    fn keyboard(&mut self, ispressed: bool, keychar: char){
        //player 1    
        if keychar == 'W'{
            if ispressed{
                self.p1.dir = Dir::UP;
            }else{
                self.p1.dir = Dir::STANDING;
            }
        }
        else if keychar == 'S'{
            if ispressed{
                self.p1.dir = Dir::DOWN;
            }else{
                self.p1.dir = Dir::STANDING;
            }
        }
        //player 2 controls
        if keychar == 'I'{
            if ispressed{
                self.p2.dir = Dir::UP;
            }else{
                self.p2.dir = Dir::STANDING;
            }
        }
        if keychar == 'K'{
            if ispressed{
                self.p2.dir = Dir::DOWN;
            }else{
                self.p2.dir = Dir::STANDING;
            }
        }         
    }    
}

fn main(){
    gamewindow::makegame("hello world", [GAMERECT[2] as u32, GAMERECT[3] as u32], 30, 30, &mut Ponggame::newstandart());
}
