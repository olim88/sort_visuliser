//sorting algorithms
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};


//check
pub fn check_sorted(values : &Vec<i32>) -> bool{
    let mut sorted  = true;
    let mut old_var: i32 = values[0];
    for value in values{
        if value >= &old_var{
            old_var = *value;
            continue;
        }
        else{
            sorted = false;
        }
    }
    sorted
}


//algorithms


//bogo sort (random)
pub struct BogoSort{
    pub values: Vec<i32>,
    start_values: Vec<i32>,
    index : usize
}
impl  BogoSort {
    pub fn new(start_values: &Vec<i32>) -> BogoSort{
        BogoSort{ values: start_values.to_vec(), start_values : start_values.to_vec(), index: 0 }
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>){
        let mut updated = Vec::new();
        loop{
            let mut out:(Vec<i32>,Vec<usize>) = self.do_step();
            updated.append( &mut out.1);
            //once one loop is done break
            if self.index == 0{
                break;
            }
        }
        
        (self.values.clone(),updated)

    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>){ //returns values and the indexs updated
        let chosen_index;
        if self.start_values.len() == 1 {
            chosen_index = 0;
        }
        else{
            chosen_index = rand::thread_rng().gen_range(1..self.start_values.len());
        }

        self.values[self.index] = self.start_values[chosen_index];
        self.start_values.remove(chosen_index);
        let updated_index = self.index;
        self.index += 1;
        //if run out of values reset
        if self.start_values.len()== 0 {
            self.start_values = self.values.clone();
            self.index = 0;
        }

        (self.values.clone(),vec![updated_index])
    }
    
}
//merge sort
pub struct MergeSort{
    pub values: Vec<i32>,
    group_size: usize,
    index : usize,

}

impl MergeSort{
    pub fn new(start_values: &Vec<i32>) -> MergeSort{        
        MergeSort{ values: start_values.to_vec(), group_size: 1 , index: 0 }
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>){
       let mut updated = Vec::new();
        loop {
            //do a single step of the sort
            let mut out = self.do_step();
            updated.append(&mut out.1);
            //if a whole loop is done stop looping
            if self.index == 0{
                break;
            }
        }
        
        (self.values.clone(),updated)
    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>){

        let len = self.values.len() ;
        
        let start_index = self.index;
        let mid_index = self.index + self.group_size;
        let mut end_index = self.index + 2 * self.group_size;
        if end_index > len {
            end_index = len;
        }
        
        MergeSort::merge(&mut self.values, start_index, mid_index, end_index);        

        self.index += 2 * self.group_size;

        if !(self.index < len - self.group_size){
            self.group_size *= 2;
            self.index = 0;
        }

        (self.values.clone(),(start_index..end_index).collect::<Vec<usize>>())
    }


    fn merge(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
        let mut left = Vec::new();
        left.extend_from_slice(&nums[start..mid]);
        let mut right = Vec::new();
        right.extend_from_slice(&nums[mid..end]);
        let mut i = 0;
        let mut j = 0;
        let mut k = start;
    
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                nums[k] = left[i];
                i += 1;
            } else {
                nums[k] = right[j];
                j += 1;
            }
            k += 1;
        }    
        while i < left.len() {
            nums[k] = left[i];
            i += 1;
            k += 1;
        }    
        while j < right.len() {
            nums[k] = right[j];
            j += 1;
            k += 1;
        }
    }
    
}
//buble sort
pub struct BubbleSort{
    pub values: Vec<i32>,
    index: usize
}
impl BubbleSort {
    pub fn new(start_values: &Vec<i32>)->BubbleSort{
        BubbleSort { values: start_values.to_vec(),index: 0 }
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>){
        let mut updated = Vec::new();
        loop {
            //do a single buble sort
            let mut out = self.do_step();
            updated.append(&mut out.1);
            //if a whole loop is done stop looping
            if self.index == 0{
                break;
            }
        }
        
        (self.values.clone(),updated)
    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>){ //returns values and the indexs updated
        let first_index = self.index;
        let second_index = self.index + 1;
        //if need swap swap values
        if self.values[first_index] > self.values[second_index]{
            (self.values[first_index], self.values[second_index]) = (self.values[second_index], self.values[first_index]);

        }
        //increase the index
        self.index += 1;
        //if at end of values go back to start
        if self.index == self.values.len() - 1{
            self.index = 0;
        }

        (self.values.clone(),vec![first_index,second_index])
    }
}



#[derive(Debug, Copy, Clone)]
pub enum AlgorithmTypes {
    Bogo,
    Merge,
    Bubble,

}
impl AlgorithmTypes {
    pub fn turn(&self) -> Self {
        use AlgorithmTypes::*;
        match *self {
            Bogo => Merge,
            Merge => Bubble,
            Bubble => Bogo,
        }
    }
}
pub struct Algorithms{
    pub current_type: AlgorithmTypes,
    pub bogo : BogoSort,
    pub merge: MergeSort,
    pub bubble: BubbleSort,
}
impl Algorithms {
    pub fn new(start_values: Vec<i32>) -> Algorithms{
        Algorithms { current_type: AlgorithmTypes::Bogo, 
            bogo: BogoSort::new(&start_values),
            merge: MergeSort::new(&start_values),
            bubble: BubbleSort::new(&start_values)
        }
    }

    pub fn reset_values(&mut self,new_values: &Vec<i32>){
        self.bogo = BogoSort::new(new_values);
        self.merge = MergeSort::new(new_values); 
        self.bubble = BubbleSort::new(new_values); 
    }
}
