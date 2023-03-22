//controls inputs to control visuals
use piston::GenericEvent;
use rand::prelude::*;
use crate::algorithms::{check_sorted,Algorithms,AlgorithmTypes};

pub struct ControllerSettings{

}
impl ControllerSettings{
    pub fn new() ->ControllerSettings{
        ControllerSettings{}
    }
}


pub struct MainController{
    pub settings: ControllerSettings,
    pub display_values: Vec<i32>,
    pub updated_values: Vec<usize>,
    pub sorted: bool,
    max_value : i32,
}
impl MainController{
    pub fn new(start_values: &Vec<i32>) -> MainController {
        MainController {
            settings: ControllerSettings::new(),
            display_values: start_values.to_vec(),
            updated_values: Vec::new(),
            sorted: false,
            max_value: 100,
        }
    }
    pub fn event<E: GenericEvent>(&mut self, e: &E,algorithm: &mut Algorithms){
        use piston::input::{Button, Key};


        //change sorting algorithms
        if let Some(Button::Keyboard(key)) = e.press_args() {            
            match key {
                Key::Minus => algorithm.current_type = algorithm.current_type.turn(),
                Key::D1 => self.update_current_sort_type(AlgorithmTypes::Bogo, algorithm),
                Key::D2 => self.update_current_sort_type(AlgorithmTypes::Merge, algorithm),
                Key::D3 => self.update_current_sort_type(AlgorithmTypes::Bubble, algorithm),
                _ => {}
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {            
            match key {
                Key::Space => self.do_current_sort(algorithm),
                Key::Tab => self.do_current_step(algorithm),
                Key::R => self.re_shuffle_values(algorithm),
                _ => {}
            }
        }
        //scroll number of values
        if let Some(args) = e.mouse_scroll_args(){
            if args[1] == 1.0{
                //add random value
                let new_value: i32 = rand::thread_rng().gen_range(1..self.max_value);
                self.display_values.push(new_value);
                //reset algorithms
                algorithm.reset_values(&self.display_values);
                //if this unsorts the array reset values
                if !check_sorted(&self.display_values){
                    self.sorted = false;
                    self.updated_values = vec![self.display_values.len()-1];
                }
            }
            else if  args[1] == -1.0 && self.display_values.len()>1 {
                //remove value
                self.display_values.pop();
                algorithm.reset_values(&self.display_values);
                //if this makes the list sorted agin set the state
                if check_sorted(&self.display_values){
                    self.sorted = true;
                }

         
            }
        }
    }
    fn update_current_sort_type(&self,new_type: AlgorithmTypes, algorithm: &mut Algorithms){
        //makesure the have the same values
        algorithm.reset_values(&self.display_values);
        //set the new type
        algorithm.current_type = new_type;
    }
    pub fn do_current_sort(&mut self, algorithm: &mut Algorithms)
    {   

        //if the data is not sorted continue sort
        if !check_sorted(&self.display_values){
            self.sorted = false;
            match algorithm.current_type {
                AlgorithmTypes::Bogo => (self.display_values,self.updated_values) = algorithm.bogo.do_sort().clone(),
                AlgorithmTypes::Merge => (self.display_values,self.updated_values) = algorithm.merge.do_sort().clone(),
                AlgorithmTypes::Bubble => (self.display_values,self.updated_values) = algorithm.bubble.do_sort().clone(),
            };
            if check_sorted(&self.display_values){
                self.sorted = true;
            }
        }  
   
            
    }
    fn do_current_step(&mut self, algorithm: &mut Algorithms)
    {
        //if the data is not sorted continue sort
        if !check_sorted(&self.display_values){
            self.sorted = false;
            match algorithm.current_type {
                AlgorithmTypes::Bogo => (self.display_values,self.updated_values) = algorithm.bogo.do_step().clone(),
                AlgorithmTypes::Merge => (self.display_values,self.updated_values) = algorithm.merge.do_step().clone(),
                AlgorithmTypes::Bubble => (self.display_values,self.updated_values) = algorithm.bubble.do_step().clone(),
            };
            if check_sorted(&self.display_values){
                self.sorted = true;
            }
        }   
   
            
    }
    pub fn re_shuffle_values(&mut self, algorithm: &mut Algorithms)
    {
        //create new random list of values
        let count = self.display_values.len();
        self.display_values = Vec::new();
        for _i in 0..count{
            self.display_values.push(rand::thread_rng().gen_range(1..self.max_value));
        }
        algorithm.reset_values(&self.display_values);
        //reset sorted status and change values to all
        self.sorted = false;
        self.updated_values = (0..count).collect::<Vec<usize>>();
    }
}

