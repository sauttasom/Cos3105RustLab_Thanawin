use std::io;

fn get_greeting() -> String {
    let greeting = "Hello, Rust!";
    let s1 = String::from("hello");
    let result = greeting.to_string();
    return s1;
}

fn calculate_grade(lab_score: u32, exam_score: u32) -> String {
    let lab_weight = 0.4;
    let exam_weight = 0.6;
    
    
    let lab_point = lab_score as f64 * lab_weight;
    let exam_point = exam_score as f64 * exam_weight;
    println!("lab_point: {}", lab_point);
    println!("exam_point: {}", exam_point);
      
    let total_score = lab_point + exam_point;
    println!("total_score: {}", total_score);
    let grade = match total_score {
        90.0..=100.0 => "A",
        85.0..=89.0 => "B+",
        80.0..=84.0 => "B",
        65.0..=79.0 => "C+",
        60.0..=64.0 => "C",
        55.0..=59.0 => "D+",
        50.0..=54.0 => "D",
        _ => "F",
    };
    

    return  grade.to_string();
}

fn main() {
    
    let mut lab_score: u32 = 0;
    let mut exam_score: u32 = 0;
    
    let mut lab_score_input = String::new();
    let mut exam_score_input = String::new();
    
    println!("Input  lab score");
    io::stdin().read_line(&mut lab_score_input).expect("Failed to  line lab_score_input");
    println!("Input  exam score");
    io::stdin().read_line(&mut exam_score_input).expect("Failed to  line exam_score_input");
    
    lab_score = lab_score_input.trim().parse().expect("Failed to parse lab score");
    exam_score = exam_score_input.trim().parse().expect("Failed to parse exam score");
    
    println!("Lab Score: {}", lab_score);
    println!("Exam Score: {}", exam_score);
    
    println!("calculate Grade");
    let grade = calculate_grade(lab_score, exam_score);
     println!("Grade: {}", grade);
    
    // let fivedata = calculate_grade(lab_score, exam_score);
    // println!("test: {}", fivedata);
}
