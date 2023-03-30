//data display
use graphics::types::Color;
use graphics::{CharacterCache, Context, Graphics,Text,Transformed,Rectangle};

use crate::main_cotroller::HudValues;

pub struct RenderSettings{
    pub position: [f64; 2],
    pub size: [f64; 2],
    pub font_size: u32,
    pub line_spaceing: u32,
    pub text_colour: Color,
    pub bar_colour: Color,
    pub bar_update_colour: Color,
    pub bar_sorted_colour: Color,
    pub bar_padding_percentage: f64,
}
impl RenderSettings {
    pub fn new() -> RenderSettings{
        RenderSettings { 
            position: [0.0,0.0], 
            size: [640.0,480.0], 
            font_size: 16,
            line_spaceing: 4,
            text_colour: [1.0, 1.0, 1.0, 1.0],
            bar_colour: [0.3, 0.0, 0.3, 0.8], 
            bar_update_colour: [0.3, 0.0, 0.3, 1.0], 
            bar_sorted_colour: [0.0, 0.25, 0.05, 1.0], 
            bar_padding_percentage: 0.03 }
    }
    
}
//data used for render
pub struct  Renderer{
    pub max_value: i32,
    pub ofset_value: i32,
    values: Vec<i32>,
    updated_index: Vec<usize>,
    data_sorted: bool,
    hud_values: HudValues,
    pub settings: RenderSettings,
}
//implement it
impl Renderer{
    //create new render
    pub fn new (max: i32, ofset: i32, values: Vec<i32>, settings : RenderSettings) -> Renderer{
        Renderer { max_value: (max), ofset_value: (ofset), values: (values),updated_index: (Vec::new()),data_sorted: (false),hud_values: HudValues::new(),settings : (settings), }
    }

    //draw the value bars
    pub fn draw<G:Graphics, C>(&self,glyphs: &mut C, c: &Context,g:&mut G)where
    C: CharacterCache<Texture = G::Texture>,
    {
        let line_count = self.values.len() as f64;
        //work out line size
        let line_width = self.settings.size[0] /line_count;
        let line_padding_width = line_width * self.settings.bar_padding_percentage;
        //work out base position
        let y = self.settings.position[1] + line_padding_width;
        //render lines
        for (index,value) in self.values.iter().enumerate()
        {
            //get location of the bar
            let x = self.settings.position[0]+ line_width * index as f64 + line_padding_width;
            //work out the height of the bar
            let height = (*value as f64 /self.max_value as f64 ) * (self.settings.size[1]-line_padding_width * 2.0);
            let y_ofset = self.settings.size[1] - height;
            //get colour based on if it has been updated
            let colour;
            if self.data_sorted{
                colour = self.settings.bar_sorted_colour;
            }
            else if self.updated_index.contains(&index){
                colour = self.settings.bar_update_colour;
            }
            else {
                colour = self.settings.bar_colour;
            }
            //render each line
            let rect: [f64; 4] = [x,y+y_ofset,line_width - (2.0 * line_padding_width),height];
            Rectangle::new(colour).draw(rect, &c.draw_state, c.transform, g,);
        }

        //draw text about algorithms#
        self.render_lines_text(format!("Current algorithm: {} \nComplexity: {} \nPredicted count: {} \nCurrent iterator: {} \n Total values: {}",self.hud_values.algorithm_info.name,self.hud_values.algorithm_info.complexity,self.hud_values.predicted_n,self.hud_values.current_n,self.hud_values.total_n), glyphs, c, g);
        
        
    }
    fn render_lines_text<G:Graphics, C>(&self, text: String,glyphs: &mut C, c: &Context,g:&mut G)where
    C: CharacterCache<Texture = G::Texture>,
    {
       //split text into lines
       let lines = text.lines();
       //set up render thingy
       let text_render = Text::new(self.settings.font_size);
       //render each line on new line
       for (index,line) in lines.enumerate(){
            let _err = text_render.draw(line, glyphs, &c.draw_state, c.transform.trans(0.0, ((self.settings.font_size + self.settings.line_spaceing) as usize * (index+1)) as f64), g);
       }

    }
    pub fn update_values(&mut self, new_values :&Vec<i32>,updated_value_index: &Vec<usize>,data_sorted: &bool,hud_value: HudValues ){
        self.values = new_values.to_vec();
        self.updated_index = updated_value_index.to_vec();
        self.data_sorted = *data_sorted;
        self.hud_values = hud_value;
        
    }
    pub fn update_size(&mut self, new_size:[f64;2]){
        self.settings.size = new_size;
    }
}