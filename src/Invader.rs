
use std::{cmp::min, fmt::write, io::stdout, path::Display, thread::panicking};

use ratatui::{
    buffer::Buffer, crossterm::{cursor::position, event, terminal::{disable_raw_mode, LeaveAlternateScreen}, ExecutableCommand}, layout::{Alignment, Rect}, style::{Color, Style, Stylize}, text::{Line, Text}, widgets::{Paragraph, Widget} 
};
use crate::Bullet::Bullet;
use geo::{Point, Translate};


pub struct Invader {
    position: Point,
    target: Point,
    win_height: u16,
    win_width: u16,
    speed: f64
}

pub struct InvaderWidget {
    position:(u16, u16)
}

trait Normalize {
    fn norm(&self)->f64;
    fn normalize(&mut self);
}

impl Normalize for Point {
    fn norm(& self) ->f64{
        let res = self.x()*self.x() + self.y()*self.x();
        if res<1. {
            0.
        } else {
            f64::sqrt(res)
        }
    }
    fn normalize(&mut self) {
        let norm:f64 = self.norm();
        if norm<0.1 {
            println!("{}, {}", self.x(), self.y());
            *self.x_mut()=1.;
            *self.y_mut()=1.;
            return;
        }
        if norm.is_nan() {
            stdout().execute(event::DisableMouseCapture);
            stdout().execute(LeaveAlternateScreen);
            disable_raw_mode();
            panic!("ahhhhhhhhhh");
        }
        *self.x_mut() /= norm;
        *self.y_mut() /= norm;
    }
}

impl Invader {
    pub fn new(x: f64, y: f64)->Self {
        Self {
            position: Point::new(x, y),
            speed: 1.,
            win_height:0,
            win_width: 0,
            target: Point::new(x, y)
        }
    }

    pub fn set_frame(&mut self, height: u16, width: u16) {
        self.win_height = height;
        self.win_width = width;
    }

    pub fn set_target(&mut self, x:f64, y: f64) {
        self.target.set_x(x);
        self.target.set_y(y);
    }

    pub fn gravitate(&mut self) {
        let mut direction = self.target - self.position;
        if direction.norm()<0.5 {
            // to small norm induces bad behaviour
            return;
        }
        direction.normalize();
        direction *= self.speed;

        self.position += direction;
    }

    fn fixed_location(&self) ->(u16, u16) {
        let mut x = self.position.x().trunc() as u16;
        let mut y = self.position.y().trunc() as u16;
        if x<4 {
            x = 4;
        } else if x>self.win_width-4 {
            x = self.win_width-4;
        }

        if y<3 {
            y = 3;
        } else if y>self.win_height-2 {
            y = self.win_height-2;
        }

        (x, y)
    }

    pub fn up(&mut self) ->(){
        if (self.position.y().trunc() as u16) > 3 {
            self.position = self.position.translate(0., -1.);
        } 
    }

    pub fn down(&mut self) ->(){
        if (self.position.y().trunc() as u16) < self.win_height-2{
            self.position = self.position.translate(0., 1.);
        }
    }

    pub fn right(&mut self) ->(){
        if (self.position.x().trunc() as u16) < self.win_width-4 {
            self.position = self.position.translate(1., 0.);
        }
    }
    
    pub fn left(&mut self) ->(){
        if (self.position.x().trunc() as u16) > 4 {
            self.position = self.position.translate(1., 0.);
        }
    }

    pub fn get_widget(&self)-> InvaderWidget {
        InvaderWidget {position: self.fixed_location()}
    }
    pub fn shoot(&self, projectile_buff: &mut Vec<Bullet>) {
        let mut bullet: Bullet = Bullet::from_point(&self.position);
        bullet.set_frame(self.win_height, self.win_width);
        projectile_buff.push(bullet);
    }
}

impl Widget for InvaderWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (x, y) = self.position;
        buf.set_string(x-4, y-3, "   ██   ", Style::default());
        buf.set_string(x-4, y-2, " ██████ ", Style::default());
        buf.set_string(x-4, y-1, "██ ██ ██", Style::default());
        buf.set_string(x-4, y+0, "████████", Style::default());
        buf.set_string(x-4, y+1, " █ ██ █ ", Style::default());
        buf.set_string(x-4, y+2, "█ █  █ █", Style::default());
    }
}
