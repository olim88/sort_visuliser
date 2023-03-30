//controls inputs to control visuals
use piston::GenericEvent;
use rand::prelude::*;
use crate::algorithms::{check_sorted,Algorithms,AlgorithmTypes,AlgorithmInfo};

#[derive(Clone)]
pub struct HudValues{
    pub algorithm_info: AlgorithmInfo,
    pub predicted_n: String,
    pub current_n: usize,
    pub total_n: usize
}
impl HudValues {
    pub fn new() -> HudValues{
        HudValues { algorithm_info: AlgorithmInfo { name: "".to_string(), complexity: "".to_string() },predicted_n: "".to_string(), current_n: 0,total_n: 0 }
    }
}

pub struct ControllerSettings{

}
impl ControllerSettings{
    pub fn new() ->ControllerSettings{
        ControllerSettings{}
    }
}


pub struct MainController{
    pub settings: ControllerSettings,
    pub hud_values: HudValues,
    pub display_values: Vec<i32>,
    pub updated_values: Vec<usize>,
    pub sorted: bool,
    max_value : i32,
}
impl MainController{
    pub fn new(start_values: &Vec<i32>) -> MainController {
        MainController {
            settings: ControllerSettings::new(),
            hud_values: HudValues::new(),
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
                Key::D4 => self.update_current_sort_type(AlgorithmTypes::InsertionSort, algorithm),
                Key::D5 => self.update_current_sort_type(AlgorithmTypes::HeapSort, algorithm),
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
                //update the prediction values
                self.updated_hud_predicted(algorithm);
            }
            else if  args[1] == -1.0 && self.display_values.len()>1 {
                //remove value
                self.display_values.pop();
                algorithm.reset_values(&self.display_values);
                //if this makes the list sorted agin set the state
                if check_sorted(&self.display_values){
                    self.sorted = true;
                }
                //update the prediction values
                self.updated_hud_predicted(algorithm);

         
            }
        }
    }
    fn update_current_sort_type(&mut self,new_type: AlgorithmTypes, algorithm: &mut Algorithms){
        //makesure the have the same values
        algorithm.reset_values(&self.display_values);
        //set the new type
        algorithm.current_type = new_type;
        //reset the hud
        self.update_hud_info(algorithm);
    }
    pub fn do_current_sort(&mut self, algorithm: &mut Algorithms)
    {   

        //if the data is not sorted continue sort
        if !check_sorted(&self.display_values){
            let mut n_change= 0;
            self.sorted = false;
            match algorithm.current_type {
                AlgorithmTypes::Bogo => (self.display_values,self.updated_values,n_change) = algorithm.bogo.do_sort().clone(),
                AlgorithmTypes::Merge => (self.display_values,self.updated_values,n_change) = algorithm.merge.do_sort().clone(),
                AlgorithmTypes::Bubble => (self.display_values,self.updated_values,n_change) = algorithm.bubble.do_sort().clone(),
                AlgorithmTypes::InsertionSort => (self.display_values,self.updated_values,n_change) = algorithm.insertion.do_sort().clone(),
                AlgorithmTypes::HeapSort => (self.display_values,self.updated_values,n_change) = algorithm.heap.do_sort().clone(),
            };
            if check_sorted(&self.display_values){
                self.sorted = true;
            }

            self.hud_values.current_n += n_change;
        }  
   
            
    }
    fn do_current_step(&mut self, algorithm: &mut Algorithms)
    {
        let mut n_change = 0;
        //if the data is not sorted continue sort
        if !check_sorted(&self.display_values){
            self.sorted = false;
            match algorithm.current_type {
                AlgorithmTypes::Bogo => (self.display_values,self.updated_values,n_change) = algorithm.bogo.do_step().clone(),
                AlgorithmTypes::Merge => (self.display_values,self.updated_values,n_change) = algorithm.merge.do_step().clone(),
                AlgorithmTypes::Bubble => (self.display_values,self.updated_values,n_change) = algorithm.bubble.do_step().clone(),
                AlgorithmTypes::InsertionSort => (self.display_values,self.updated_values,n_change) = algorithm.insertion.do_step().clone(),
                AlgorithmTypes::HeapSort => (self.display_values,self.updated_values,n_change) = algorithm.heap.do_step().clone(),
            };
            if check_sorted(&self.display_values){
                self.sorted = true;
            }
        }  
        self.hud_values.current_n += n_change;

   
            
    }
    fn update_hud_info(&mut self, algorithm: &mut Algorithms){
        //get the text for the current sort
        match algorithm.current_type {
            AlgorithmTypes::Bogo => (self.hud_values.algorithm_info.name,self.hud_values.algorithm_info.complexity) = (algorithm.bogo.name.clone(),algorithm.bogo.complexity.clone()),
            AlgorithmTypes::Merge => (self.hud_values.algorithm_info.name,self.hud_values.algorithm_info.complexity) = (algorithm.merge.name.clone(),algorithm.merge.complexity.clone()),
            AlgorithmTypes::Bubble => (self.hud_values.algorithm_info.name,self.hud_values.algorithm_info.complexity) = (algorithm.bubble.name.clone(),algorithm.bubble.complexity.clone()),
            AlgorithmTypes::InsertionSort => (self.hud_values.algorithm_info.name,self.hud_values.algorithm_info.complexity) = (algorithm.insertion.name.clone(),algorithm.insertion.complexity.clone()),
            AlgorithmTypes::HeapSort => (self.hud_values.algorithm_info.name,self.hud_values.algorithm_info.complexity) = (algorithm.heap.name.clone(),algorithm.heap.complexity.clone()),
        };
        //update the prediction values
        self.updated_hud_predicted(algorithm);

    }
    fn updated_hud_predicted(&mut self, algorithm: &mut Algorithms){
        //reset value for current n
        self.hud_values.current_n  = 0;
        //set the total value
        self.hud_values.total_n = self.display_values.len();
        //calculate the complexitys for current sort
        let best;
        let avarage;
        let worst;
        match algorithm.current_type {
            AlgorithmTypes::Bogo => (best,avarage,worst) = algorithm.bogo.get_predicted_complexity(),
            AlgorithmTypes::Merge => (best,avarage,worst) = algorithm.merge.get_predicted_complexity(),
            AlgorithmTypes::Bubble => (best,avarage,worst) = algorithm.bubble.get_predicted_complexity(),
            AlgorithmTypes::InsertionSort => (best,avarage,worst) = algorithm.insertion.get_predicted_complexity(),
            AlgorithmTypes::HeapSort => (best,avarage,worst) = algorithm.heap.get_predicted_complexity(),
        };
        self.hud_values.predicted_n = format!("Best: {:.2}, Avarage: {:.2}, Worst: {:.2}",best,avarage,worst);

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
        //reset count
        self.hud_values.current_n  = 0;
    }
}

