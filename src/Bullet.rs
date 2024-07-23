use geo::{Point, Translate};
pub struct Bullet {
    position: Point,
    win_height: u16,
    win_width: u16,
    offscreen: bool
}

impl Bullet {
    pub fn new(x: f64, y: f64)->Self {
        Self {
            position: Point::new(x, y),
            win_height:0,
            win_width: 0,
            offscreen: false
        }
    }

    pub fn from_point(position: &Point)->Self {
        Self::new(position.x(), position.y())
    }

    pub fn set_frame(&mut self, height: u16, width: u16) {
        self.win_height = height;
        self.win_width = width;
    }

    pub fn up(&mut self) ->(){
        if (self.position.y().trunc() as u16) > 0 {
            self.position = self.position.translate(0., -1.);
        } else {
            self.offscreen=true;
        }
    }

    pub fn down(&mut self) ->(){
        if (self.position.y().trunc() as u16) < self.win_height{
            self.position = self.position.translate(0., 1.);
        } else {
            self.offscreen=true;
        }
    }

    pub fn right(&mut self) ->(){
        if (self.position.x().trunc() as u16) < self.win_width {
            self.position = self.position.translate(1., 0.);
        } else {
            self.offscreen=true;
        }
    }
    
    pub fn left(&mut self) ->(){
        if (self.position.x().trunc() as u16) > 0 {
            self.position = self.position.translate(1., 0.);
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

impl Widget for BulletSprite<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            self.position.x().trunc() as u16,
            self.position.y().trunc() as u16,
            "█",
            Style::default().fg(Color::Red
        ));
    }
}