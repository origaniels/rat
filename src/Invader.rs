pub struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16 
}

pub struct Invader {
    position: Point,
    win_height: u16,
    win_width: u16
}

pub struct InvaderWidget<'a> {
    position: &'a Point
}

impl Invader {
    pub fn new(x: u16, y: u16)->Self {
        Self {
            position: Point {x: x, y: y},
            win_height:0,
            win_width: 0
        }
    }

    pub fn set_frame(&mut self, height: u16, width: u16) {
        self.win_height = height;
        self.win_width = width;
    }

    pub fn shoot(&self, projectile_buff: &mut Vec<Bullet>) {
        let mut bullet: Bullet = Bullet::new(self.position.x+3, self.position.y);
        bullet.set_frame(self.win_height, self.win_width);
        projectile_buff.push(bullet);
    }

    pub fn up(&mut self) ->(){
        if self.position.y>0 {
            self.position.y -= 1;
        }
    }

    pub fn down(&mut self) ->(){
        if self.position.y< self.win_height-8{
            self.position.y += 1;
        }
    }

    pub fn right(&mut self) ->(){
        if self.position.x<self.win_width-8 {
            self.position.x += 1;
        }
    }
    
    pub fn left(&mut self) ->(){
        if self.position.x>0 {
            self.position.x -= 1;
        }
    }
    pub fn get_widget(&self)-> InvaderWidget {
        InvaderWidget {position: &self.position}
    }
}

use std::vec;

use ratatui::{
    buffer::Buffer, crossterm::cursor::position, layout::{Alignment, Rect}, style::{Color, Style, Stylize}, text::{Line, Text}, widgets::{Paragraph, Widget} 
};

use crate::Bullet::Bullet;

impl Widget for InvaderWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(self.position.x, self.position.y+0, "   ██   ", Style::default());
        buf.set_string(self.position.x, self.position.y+1, " ██████ ", Style::default());
        buf.set_string(self.position.x, self.position.y+2, "██ ██ ██", Style::default());
        buf.set_string(self.position.x, self.position.y+3, "████████", Style::default());
        buf.set_string(self.position.x, self.position.y+4, " █ ██ █ ", Style::default());
        buf.set_string(self.position.x, self.position.y+5, "█ █  █ █", Style::default());
    }
}
