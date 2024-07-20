pub struct Bullet {
    position: Point,
    win_height: u16,
    win_width: u16,
    offscreen: bool
}

impl Bullet {
    pub fn new(x: u16, y: u16)->Self {
        Self {
            position: Point {x: x, y: y},
            win_height:0,
            win_width: 0,
            offscreen: false
        }
    }

    pub fn set_frame(&mut self, height: u16, width: u16) {
        self.win_height = height;
        self.win_width = width;
    }

    pub fn up(&mut self) ->(){
        if self.position.y>0 {
            self.position.y -= 1;
        } else {
            self.offscreen=true;
        }
    }

    pub fn down(&mut self) ->(){
        if self.position.y< self.win_height{
            self.position.y += 1;
        } else {
            self.offscreen=true;
        }
    }

    pub fn right(&mut self) ->(){
        if self.position.x<self.win_width {
            self.position.x += 1;
        } else {
            self.offscreen=true;
        }
    }
    
    pub fn left(&mut self) ->(){
        if self.position.x>0 {
            self.position.x -= 1;
        } else {
            self.offscreen=true;
        }
    }

    pub fn offscreen(&self)->bool {
        return self.offscreen;
    }

    pub fn get_widget(&self)-> BulletSprite {
        BulletSprite {position: &self.position}
    }
}

pub struct  BulletSprite<'a> {
    position: &'a Point
}

use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Style}, widgets::Widget
};

use crate::Invader::Point;

impl Widget for BulletSprite<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(self.position.x, self.position.y+0, "█", Style::default().fg(Color::Red));
    }
}