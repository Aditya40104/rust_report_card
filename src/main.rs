use std::fs::File;
use std::io::{self, BufWriter};
use std::process::Command;
use std::path::Path;
use printpdf::*;

/// Calculates the average marks
fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}

/// Assigns a grade based on the average
fn assign_grade(avg: f32) -> char {
    if avg >= 90.0 {
        'A'
    } else if avg >= 75.0 {
        'B'
    } else if avg >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn main() {
    // Input student details
    let mut name = String::new();
    let mut total_marks_str = String::new();
    let mut num_subjects_str = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks_str).unwrap();
    let total_marks: f32 = total_marks_str.trim().parse().expect("Invalid number");

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects_str).unwrap();
    let num_subjects: u32 = num_subjects_str.trim().parse().expect("Invalid number");

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    // Create PDF with better formatting
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    // Line formatting
    let mut y_position = 270.0;
    let line_spacing = 10.0;
    let lines: Vec<String> = vec![
    "📘 Student Report Card".to_string(),
    format!("Name: {}", name),
    format!("Total Marks: {}", total_marks),
    format!("Subjects: {}", num_subjects),
    format!("Average: {:.2}", average),
    format!("Grade: {}", grade),
    ];


    for line in &lines {
        current_layer.use_text(line, 14.0, Mm(20.0), Mm(y_position), &font);
        y_position -= line_spacing;
    }

    // Save PDF
    let pdf_path = "report_card.pdf";
    let output = File::create(pdf_path).unwrap();
    doc.save(&mut BufWriter::new(output)).unwrap();

    println!("\n✅ PDF '{}' generated successfully.", pdf_path);

    // Auto-open (WSL or Ubuntu)
    let absolute_path = Path::new(pdf_path).canonicalize().unwrap();
    let file_url = format!("file://{}", absolute_path.display());

    // Try explorer.exe for WSL (Windows), fallback to xdg-open
    let open_result = if cfg!(target_os = "linux") {
        // Try Windows browser (WSL)
        Command::new("explorer.exe").arg(pdf_path).spawn()
            .or_else(|_| {
                // Try Linux xdg-open
                Command::new("xdg-open").arg(&file_url).spawn()
            })
    } else {
        // Default fallback
        Command::new("xdg-open").arg(&file_url).spawn()
    };

    if open_result.is_err() {
        println!("⚠️ Could not open PDF automatically. Please open it manually.");
    }
}
