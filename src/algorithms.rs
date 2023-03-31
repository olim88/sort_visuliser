//sorting algorithms
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rust_math::num::{self, factorial};

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
    index : usize,
    pub name: String,
    pub complexity: String,
}
impl  BogoSort {
    pub fn new(start_values: &Vec<i32>) -> BogoSort{
        BogoSort{ values: start_values.to_vec(), start_values : start_values.to_vec(), index: 0, name: "Bogo sort".to_string(),complexity:"best: O(n), average: O(n+1)!, worst: O(-1)".to_string() }
    }
    pub fn get_predicted_complexity(&self) ->(f32,f32,f32){
        let n = self.values.len() as f32;
        let best: f32 = n;
        let avarage;
        if n > 30.0{//make sure the factorial result is not to big
            avarage = f32::INFINITY;
        }
        else{
            avarage = factorial(n as i32 + 1 ) as f32;
        }
        let worst: f32 = f32::INFINITY;

        (best,avarage,worst)
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>,usize){
        let mut updated = Vec::new();
        let mut n_change: usize = 0;
        loop{
            let mut out:(Vec<i32>,Vec<usize>, usize) = self.do_step();
            updated.append( &mut out.1);
            n_change += out.2;
            //once one loop is done break
            if self.index == 0{
                break;
            }
        }
        
        (self.values.clone(),updated,n_change)

    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>,usize){ //returns values and the indexs updated
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

        (self.values.clone(),vec![updated_index],1)
    }
    
}
//merge sort
pub struct MergeSort{
    pub values: Vec<i32>,
    group_size: usize,
    index : usize,
    pub name: String,
    pub complexity: String,
    

}

impl MergeSort{
    pub fn new(start_values: &Vec<i32>) -> MergeSort{        
        MergeSort{ values: start_values.to_vec(), group_size: 1 , index: 0, name: "Merge sort".to_string(),complexity:"best: O(nlogn), average: O(nlogn), worst: O(nlogn)".to_string() }
    }
    pub fn get_predicted_complexity(&self) ->(f32,f32,f32){
        let n = self.values.len() as f32;
        let best = n * n.log2();
        let avarage =  n * n.log2();
        let worst =  n * n.log2() ;

        (best,avarage,worst)
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>,usize){
       let mut updated = Vec::new();
       let mut n_count = 0;
        loop {
            //do a single step of the sort
            let mut out = self.do_step();
            updated.append(&mut out.1);
            n_count += out.2;
            //if a whole loop is done stop looping
            if self.index == 0{
                break;
            }
        }
        
        (self.values.clone(),updated,n_count)
    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>,usize){

        let len = self.values.len() ;
        
        let start_index = self.index;
        let mid_index = self.index + self.group_size;
        let mut end_index = self.index + 2 * self.group_size;
        if end_index > len {
            end_index = len;
        }
        
        let n_count = MergeSort::merge(&mut self.values, start_index, mid_index, end_index);        

        self.index += 2 * self.group_size;

        if !(self.index < len - self.group_size){
            self.group_size *= 2;
            self.index = 0;
        }

        (self.values.clone(),(start_index..end_index).collect::<Vec<usize>>(),n_count)
    }


    fn merge(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) -> usize {
        let mut n_count = 0;
        let mut left = Vec::new();
        left.extend_from_slice(&nums[start..mid]);
        let mut right = Vec::new();
        right.extend_from_slice(&nums[mid..end]);
        let mut i = 0;
        let mut j = 0;
        let mut k = start;
    
        while i < left.len() && j < right.len() {
            n_count += 1;
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
            n_count += 1;
            nums[k] = left[i];
            i += 1;
            k += 1;
        }    
        while j < right.len() {
            n_count += 1;
            nums[k] = right[j];
            j += 1;
            k += 1;
        }
        //return the amount of values edited
        n_count
    }
    
}
//buble sort
pub struct BubbleSort{
    pub values: Vec<i32>,
    index: usize,
    pub name: String,
    pub complexity: String,
    
}
impl BubbleSort {
    pub fn new(start_values: &Vec<i32>)->BubbleSort{
        BubbleSort { values: start_values.to_vec(),index: 0, name: "Bubble sort".to_string(),complexity:"best: O(n), average: O(n^2), worst: O(n^2)".to_string() }
    }
    pub fn get_predicted_complexity(&self) ->(f32,f32,f32){
        let n = self.values.len() as f32;
        let best = n ;
        let avarage =  n * n;
        let worst =  n * n ;

        (best,avarage,worst)
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>,usize){
        let mut updated = Vec::new();
        let mut n_count = 0;
        loop {
            //do a single buble sort
            let mut out = self.do_step();
            updated.append(&mut out.1);
            n_count += out.2;
            //if a whole loop is done stop looping
            if self.index == 0{
                break;
            }
        }
        
        (self.values.clone(),updated,n_count)
    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>,usize){ //returns values and the indexs updated
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

        (self.values.clone(),vec![first_index,second_index],1)
    }
}

//insertion sort
pub struct InsertionSort{
    pub values: Vec<i32>,
    index: usize,
    look_index: usize,
    pub name: String,
    pub complexity: String,
}
impl InsertionSort{
    pub fn new(start_values: &Vec<i32>)->InsertionSort{
        InsertionSort { values: start_values.to_vec(),index: 1,look_index: 0, name: "Insertion sort".to_string(),complexity:"best: O(n), average: O(n^2), worst: O(n^2)".to_string() }
    }
    pub fn get_predicted_complexity(&self) ->(f32,f32,f32){
        let n = self.values.len() as f32;
        let best = n ;
        let avarage =  n * n;
        let worst =  n * n ;

        (best,avarage,worst)
    }
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>,usize){//if it dose one loop it compleats the sort so just return one step i think this is the best way to go about it
        self.do_step()
    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>,usize){ //returns values and the indexs updated
        let mut n_count = 0;
        self.look_index = self.index;
        let old_index = self.index;

        while self.look_index > 0 && self.values[self.look_index-1] > self.values[self.look_index] {
            self.values.swap(self.look_index,self.look_index-1);    
            self.look_index -= 1;
            n_count += 2;
        }       
        let new_index = self.look_index;
        self.index += 1;   

        (self.values.clone(),vec![new_index,old_index],n_count)
    }
}
//heap sort
pub struct HeapSort{
    pub values: Vec<i32>,
    index : usize,
    second_half: bool,
    need_heapify: bool,
    heapify_max_range: usize,
    heapify_parent_node: usize,
    pub name: String,
    pub complexity: String,
 
}
//heap sort
impl HeapSort{
    pub fn new(start_values: &Vec<i32>)->HeapSort{
        HeapSort { values: start_values.to_vec(), index:start_values.len()/2,second_half :false,need_heapify:false,heapify_max_range:0,heapify_parent_node:0, name: "Heap sort".to_string(),complexity:"best: O(nlogn), average: O(nlogn), worst: O(nlogn)".to_string() }
    }
    pub fn get_predicted_complexity(&self) ->(f32,f32,f32){
        let n = self.values.len() as f32;     
        let best = n * n.log2();
        let avarage =  n * n.log2();
        let worst =  n * n.log2() ;

        (best,avarage,worst)
    }
   
    pub fn do_sort(&mut self) -> (Vec<i32>,Vec<usize>,usize){
        let mut updated = Vec::new();
        let mut n_count = 0;
        //loop round until it is ready to move on to the next value and stoped recertion
        let mut out = self.do_step();
        updated.append(&mut out.1); 
        n_count += out.2;
        while self.need_heapify {     
            let mut out = self.do_step();        
            updated.append(&mut out.1);  
            n_count += out.2;                      
        }

        (self.values.clone(),updated.clone(),n_count)
    }
    pub fn do_step(&mut self) -> (Vec<i32>,Vec<usize>,usize){ 
        //if the next step needs to be heapifing the values or moving onto the next value 
        if self.need_heapify{
            //stop this from needing to be the next step
            self.need_heapify = false;
            //get the largest value out of yeapify_1 +0 +1 and + 2
            let mut largest = self.heapify_parent_node;
            let left = 2 * self.heapify_parent_node + 1;
            let right = 2 * self.heapify_parent_node + 2;
            if left < self.heapify_max_range && self.values[left] > self.values[largest] {
                largest = left;
            }
            if right < self.heapify_max_range && self.values[right] > self.values[largest] {
                largest = right;
            }
            //if the largest is one of the left and right values swap the left or right value with i
            if largest != self.heapify_parent_node {
                self.values.swap(self.heapify_parent_node, largest);
                let updated = vec![self.heapify_parent_node,largest];
                self.heapify_parent_node = largest;
                self.need_heapify = true;
                return (self.values.clone(),updated,1);
            }            
        }
        else{
            let mut updated: Vec<usize> = Vec::new();
            let mut n_count = 0;
            if !self.second_half{
                //if we are looping though the first part of the list
                self.heapify_max_range = self.values.len();
                self.heapify_parent_node = self.index;
                self.need_heapify = true;           
            }
            else {
                //if we are looping though the whole list
                //swap the first value with the value in at the index
                self.values.swap(0,self.index);
                updated = vec![0,self.index];
                n_count += 1;
                //get ready to heapify again
                self.heapify_max_range = self.index;
                self.heapify_parent_node = 0;
                self.need_heapify = true; 
                
            }
            //when finished first half go onto second
            if self.index == 0{
                self.index = self.values.len();
                self.second_half = true;
            }
            self.index -= 1;
            
            return (self.values.clone(),updated,n_count);
        }
        return (self.values.clone(),Vec::new(),0);

    }
   
    
}

//info for algorithm
#[derive(Clone)]
pub struct AlgorithmInfo{
    pub name: String,
    pub complexity: String,
}


#[derive(Debug, Copy, Clone)]
pub enum AlgorithmTypes {
    Bogo,
    Merge,
    Bubble,
    InsertionSort,
    HeapSort

}
impl AlgorithmTypes {
    pub fn turn(&self) -> Self {
        use AlgorithmTypes::*;
        match *self {
            Bogo => Merge,
            Merge => Bubble,
            Bubble => InsertionSort,
            InsertionSort =>HeapSort,
            HeapSort => Bogo,
        }
    }
}
pub struct Algorithms{
    pub current_type: AlgorithmTypes,
    pub bogo : BogoSort,
    pub merge: MergeSort,
    pub bubble: BubbleSort,
    pub insertion: InsertionSort,
    pub heap: HeapSort,
}
impl Algorithms {
    pub fn new(start_values: Vec<i32>) -> Algorithms{
        Algorithms { current_type: AlgorithmTypes::Bogo, 
            bogo: BogoSort::new(&start_values),
            merge: MergeSort::new(&start_values),
            bubble: BubbleSort::new(&start_values),
            insertion: InsertionSort::new(&start_values),
            heap: HeapSort::new(&start_values),
        }
    }

    pub fn reset_values(&mut self,new_values: &Vec<i32>){
        self.bogo = BogoSort::new(new_values);
        self.merge = MergeSort::new(new_values); 
        self.bubble = BubbleSort::new(new_values); 
        self.insertion = InsertionSort::new(new_values);
        self.heap = HeapSort::new(new_values);
    }
}
